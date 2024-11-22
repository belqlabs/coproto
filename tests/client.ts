import { createConnection, Socket } from "net";
import { Protocol } from "./protocol";

async function sleep(seconds: number = 1): Promise<void>{
  return new Promise((resolve, reject) => {
    setTimeout(resolve, seconds * 1000);
  })
}

export class CpClient {
  private client: Socket

  private myId: number;

  private serverId: number;

  private lastMsgId: number = 0;

  constructor(
    private options: {
    registerTimeout: number,
    messageAckTimeout: number
  }){
    this.client = createConnection('/tmp/copo_sock.sock');

    this.client.on('data', this.handleMessage.bind(this))

    setTimeout(() => {
      if(!this.myId){
      }
    }, this.options.registerTimeout)
  }

  private generateMessageId(): string{
    return `${this.myId}.${this.lastMsgId++}`
  };

  private async waitForRegistry(): Promise<void>{
    let waiting = 1;
    const now = new Date();

    while(!this.myId){
      const waitedUntil = new Date();

      if(waitedUntil.getTime() - now.getTime() > this.options.registerTimeout){
        throw new Error('[TIMEOUT] Timeout reached while waiting for server registration');
      }

      await sleep(waiting);

      waiting++;
    }

    return;
  }

  private handleMessage(data: Buffer): void {
    const named = Protocol.decodeNamedValue(data);
    console.log(named);
    const dataString = data.toString();

    if(dataString.startsWith('newId')){
      this.myId = parseInt(dataString.match(/\d+/gm)![0])
      this.serverId = parseInt(dataString.slice(0, 8));

      console.log(this.serverId);
    }

  };

  private async messageACKWaiter(messageId: string): Promise<void>{
    return new Promise((resolve, reject) => {

      setTimeout(() => {
        reject(`Timeout reached while waiting for the ACK of message: ${messageId}`)
      }, this.options.messageAckTimeout)

      this.client.on('data', (data: Buffer) => {
        const dataStr = data.toString();
        
        if(dataStr.startsWith('ACK')){
          const ackId = dataStr.match(/\d+.\d+/gm)![0]

          if(ackId === messageId){
            resolve
          }
        }
      })
    })

  }

  async sendOneMessage(message: Uint8Array | string): Promise<void> {
    await this.waitForRegistry();

    const msgId = this.generateMessageId();

    console.log(msgId);

    this.client.write(message);

    await this.messageACKWaiter(msgId);
  }
}



async function main(){

  const client = new CpClient({
    registerTimeout: 1000,
    messageAckTimeout: 1000
  });

  await client.sendOneMessage('+')
  await client.sendOneMessage('+')
  await client.sendOneMessage(':')
  await client.sendOneMessage(';')
  await client.sendOneMessage('#')
  await client.sendOneMessage('(')
  await client.sendOneMessage('-')
  await client.sendOneMessage('[')
  await client.sendOneMessage('$')
  await client.sendOneMessage('@')
  await client.sendOneMessage('{')
}

main();
