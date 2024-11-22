import { randomBytes, randomInt } from "crypto";
import { createServer, Server, Socket } from "net";

export class CpServer{
  private clients: Record<number, Socket> = {};

  private possibleClients = Array.from(
    Array(255).keys()
  )

  private server: Server;

  private bootsTrapServer(): void{
    this.server = createServer();

    this.server.on('connection', this.connectionHandler.bind(this));

    this.server.listen('/tmp/copo_sock.sock', undefined, () => {
      console.log('LISTENING');
    });

    process.on('SIGINT', () => {
      this.server.close();
    })

    process.on('SIGTERM', () => {
      this.server.close();
    })
  }

  constructor(
    private serverId: number
  ){
    this.bootsTrapServer();
  }

  private generateNewClientId(): number {
    // server id = 8 bits little endian
    // client id = 8 bits little endian
    const serverBits = this.serverId.toString(2).padStart(8, '0');

    const usedClientIds = Object.keys(this.clients).map(id => parseInt(id));

    const freeId = this.possibleClients.find(availableId => (!usedClientIds.includes(availableId)));

    if(freeId === undefined){
      throw new Error("Client overflow");
    }

    const freeIdBits = freeId.toString(2).padStart(8, '0');

    const newId = parseInt(serverBits + freeIdBits, 2);

    console.log(serverBits + freeIdBits);

    return newId;

  }

  private registerClient(socket: Socket): void {
    const newId = this.generateNewClientId();
    this.clients[newId] = socket;

    console.log(newId);

    const buff = Protocol.encodeNamedValue('NID', newId);

    console.log(buff);

    socket.write(buff);
  }

  private messageHandler(data: Buffer): void{
    const clientId = data.buffer.slice(0, 2);
    console.log(clientId);
  }

  private connectionHandler(socket: Socket): void {
    socket.on('end', () => {
      console.log(`Client disconnected`);
    });

    socket.on('data', (data) => {
      this.messageHandler(data)
      console.log(`[${new Date}] ${data.toString()}`);
    });

    this.registerClient(socket);
  }

}

const server = new CpServer(1);
