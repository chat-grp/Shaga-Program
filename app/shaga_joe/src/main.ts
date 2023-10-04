import { PublicKey } from '@solana/web3.js';
import {
  initializeShagaAccounts,
  createLender,
  createAffair,
  startRental,
  endRental,
  terminateAffair,
} from './custom';
import shagaFeePayerRaw from '../../test_keypairs/1.json';
import shagaLenderRaw from '../../test_keypairs/2.json';
import shagaClientRaw from '../../test_keypairs/3.json';
import shagaLenderTwoRaw from '../../test_keypairs/4.json';
import shagaClientTwoRaw from '../../test_keypairs/0.json';
import {
  Connection,
  Keypair,
  TransactionInstruction
} from '@solana/web3.js';
import { signAndSendLegacyTransaction, stringToUint8Array } from './utils';
import * as dotenv from "dotenv";
import BN from 'bn.js'

import { Affair, AffairPayload, AffairsList } from './generated';
dotenv.config();


const shagaFeePayer = Keypair.fromSecretKey(Uint8Array.from(shagaFeePayerRaw));
const shagaLender = Keypair.fromSecretKey(Uint8Array.from(shagaLenderRaw));
const shagaClient = Keypair.fromSecretKey(Uint8Array.from(shagaClientRaw));
const shagaLenderTwo = Keypair.fromSecretKey(Uint8Array.from(shagaLenderTwoRaw));
const shagaClientTwo = Keypair.fromSecretKey(Uint8Array.from(shagaClientTwoRaw));
const connection = new Connection(process.env.RPC_URL || "", "confirmed");

async function main() {
  let instructions: TransactionInstruction[] = [];

  // instructions.push(initializeShagaAccounts(shagaFeePayer.publicKey));
  // instructions.push(createLender(shagaLender.publicKey));

  // // // Generate some dummy data
  // const dummyIpAddress = '192.168.1.1';
  // const dummyCpuName = 'Intel Core i7-9700K';
  // const dummyGpuName = 'NVIDIA GeForce RTX 3070';
  // // Convert the strings to Uint8Array
  // const ipAddressArray = stringToUint8Array(dummyIpAddress, 15);
  // const cpuNameArray = stringToUint8Array(dummyCpuName, 64);
  // const gpuNameArray = stringToUint8Array(dummyGpuName, 64);

  // const currentTimeInSeconds = Math.floor(new Date().getTime() / 1000);
  // console.log(currentTimeInSeconds);
  // // Add 1 hour to the current time
  // const terminationTimeInSeconds = currentTimeInSeconds + 3600;
  // // Exported dummy data
  // const affairPayload: AffairPayload = {
  //   ipAddress: ipAddressArray,
  //   cpuName: cpuNameArray,
  //   gpuName: gpuNameArray,
  //   totalRamMb: 16384, // Assuming 16GB RAM for this dummy data
  //   usdcPerHour: 100000, // Assuming a dummy value of 10 USDC per hour
  //   affairTerminationTime: new BN(terminationTimeInSeconds) // Assuming a dummy timestamp value
  // };
  // instructions.push(createAffair(shagaLender.publicKey, affairPayload));

  const affairKey = new PublicKey("J2SoMdVDKfjUgUB39xCcBkvcbrKDbQ9hqhBk47WMzSUk")
  // constant
  const affairsListKey = new PublicKey("HcD1vP1TzV3Su5Tkw5EfrvAZzMAjwDG9yLce5aUGazrz")
  const affairList = await AffairsList.fromAccountAddress(connection, affairsListKey)
  console.log(affairList.pretty())
  const getAffair = await Affair.fromAccountAddress(connection, affairKey) // affairList.activeAffairs[0])
  console.log(getAffair.pretty())

  const currentTimeInSeconds = Math.floor(new Date().getTime() / 1000);
  console.log(currentTimeInSeconds);
  // Add 1 hour to the current time
  const terminationTimeInSeconds = currentTimeInSeconds + 1000;
  instructions.push(await startRental(connection, shagaClient.publicKey, affairKey, terminationTimeInSeconds));

  // instructions.push(await endRental(shagaClient.publicKey, affairKey))

  // instructions.push(await terminateAffair(connection, shagaLenderTwo.publicKey, affairKey, false))


  // await signAndSendLegacyTransaction(connection,
  //   // [shagaLender],
  //   // [shagaLenderTwo],
  //   [shagaClient],
  //   // [shagaLenderTwo],
  //   // [shagaClientTwo],
  //   // shagaLender,
  //   // shagaLenderTwo,
  //   shagaClient,
  //   // shagaLenderTwo,
  //   // shagaClientTwo,
  //   instructions
  // );
}

main()