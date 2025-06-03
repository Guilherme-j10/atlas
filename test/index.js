const net = require('net');

function sendBinaryMessage(socket, operation, payload) {
  const data = Buffer.from(JSON.stringify(payload)); // 6 bytes
  const realSize = data.length; // 6
  const realSizeBuffer = Buffer.from([realSize]); // 1 byte
  const sizeLength = realSizeBuffer.length; // 1 byte, pois realSize cabe em 1 byte

  const messageBuffer = Buffer.concat([
    Buffer.from([operation]),              // [operation]
    Buffer.from([sizeLength]),             // [size_length]
    realSizeBuffer,                        // [real_size]
    data// [payload]
  ]);
  socket.write(messageBuffer);
}

// Criar conexão TCP com o servidor Rust
const client = new net.Socket();

client.connect(8787, '127.0.0.1', () => {
  console.log('Conectado ao servidor Rust');

  // Exemplo: enviar uma mensagem com operação 1 e payload "Hello"
  sendBinaryMessage(client, 1, {
    topic: "cart",
  });

  // Opcionalmente encerrar a conexão após enviar
  //client.end();
});

client.on('data', (data) => {
  console.log('Recebido do servidor:', data);
});

client.on('close', () => {
  console.log('Conexão encerrada');
});

client.on('error', (err) => {
  console.error('Erro:', err);
});
