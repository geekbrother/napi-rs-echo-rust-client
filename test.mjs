import { EchoRustClient } from './index.js'

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

async function sendEveryNSec(seconds, client) {
  let i = 0;
  while (1) {
    i++;
    await client.send("hey hey " + i);
    await sleep(seconds * 1000);
  }
}

async function onReceive(argsnum, msg) {
  console.log("Received message on callback: " + msg);
}

let client = new EchoRustClient("http://[::1]:50051", onReceive);
sendEveryNSec(3, client);

console.log('Start testing client');
