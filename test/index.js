const net = require('net');

function sendBinaryMessage(socket, operation, payload) {
  const data = JSON.stringify(payload);
  const messageBytesLength = data.length;
  const messageBytesAmount = String(`${messageBytesLength}`.length).padStart(2, "0");

  const bufferBytesData = Buffer.alloc(parseInt(messageBytesAmount));

  const bytes = `${messageBytesLength}`.split("");
  for (let i = 0; i < bytes.length; i++) {
    bufferBytesData.writeInt8(parseInt(bytes[i]), i);
  }

  const messageBuffer = Buffer.concat([
    Buffer.from([operation]),
    Buffer.alloc(2, messageBytesAmount),
    bufferBytesData,
    Buffer.from(data)
  ]);
  socket.write(messageBuffer);
}

const client = new net.Socket();
client.connect(8787, '127.0.0.1', () => {
  console.log('Conectado ao servidor Rust');

  sendBinaryMessage(client, 1, {
    topic: "cart"
  });

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
