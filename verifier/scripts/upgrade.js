// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");
const { writeFile } = require('fs');

const NAME = "Game2048Step60CircomVerifier";
//const ADDR = "0x764ae46f345be77ef2a1f707842a9e7cffb1f2fb";
//const ADDR = "0x0720DDdB51D4544ba66aCF57c28008FcC81606Fc";
const ADDR = "0x75D9FA27b6f7D22D6D1669765B703F9E79066abF";

async function test() {
  const C = await ethers.getContractFactory(NAME);
  const contract = await C.attach(ADDR);
  // contract.verify();

  const res2 = await contract.verifyProof(
    [
      //"0x15c3fd9fee855af69d69ec77c0018ad9347866614cd5e80e318a78b60ca90d74",
      //"0x2cda0ab4de0d722949ccab93ef996a2191384d7ba705fafb2d0c61e86d97d2bb"
      "0x2acfe2b7fe3fc1e8e760a8347ece95850fc756cc3a1a5e1a8433febe50c007df",
      "0x078c15ad98b5dfa52f4cb3f582d49d844ece3f9d6029aeefbc86355de9dcce03"
    ],
    [
      [
        //"0x2d1d9d34ed6e720057199ce3f29cc167311d1b2ac754104be7c79db30bb591b0",
        //"0x2056d39820d9a658a83c1dabc0e1b51599e7e01c6a4315923b5017c2edb88cfe"
        "0x2a4ccd14969eb3f6aa77c09496c17d467e5c980b92efefbe417c7ae5c407a1f4",
        "0x2bbf3839cb39ea1e2fb5ae432a42b7e0a096e6e4992b868943d10d3a6614a749"
      ],
      [
        //"0x0aa28427983ba6a8493a824911093c411e7073ce41bf890a80815b0f023a9822",
        //"0x2dce498b05b0c0f9758cf4e7da9c30f7f0d966227dfb836e7482220a7a3caa87"
        "0x06e2e015ae05fa0f650503e48a14eb21a107cc4fa454c9168f656acdc7f37462",
        "0x1ccfa893ab660d49c9eee4d65c1b1c54176efe621a8365df1b322b49b46002e7"
      ]
    ],
    [
      //"0x0c04ce9eadbc453c0ff5e7fe30ab185764c054d3f9c50f43187d57d069ac4512",
      //"0x082103fa2ad8ee293a52dafbb1c0a47d42eafb0d13400efe27d31b980bf7a3f9"
      "0x06ad8bd621f5d3036a2795de77ce1042510a10e92311bc6150fb0193f2107ad2",
      "0x152430b4cafeffcd5a1418843fb02318025182e1ecaa8a55e14be5a3900fbc2a"
    ],
    [
      "0x0000000000000000000000000000000000000000000000000000200800000000",
      "0x0000000000000000000000000000000000000000000000886004440000500023",
      "0x00000000000000000000000000000000003c0cf3cc8f230c8f0cf3ff0ef3c333",
      "0x0000000000000000000000000000000000000000000000000000000000001a85",
      "0x0000000000000000000000000000000000000000000000000000000000000000",
      "0x000000000000000000000000000000000000000000000000000000000000003c",
      "0x00000000000000000000000000000000000000000000000000000000000001c8"
    ]
  );
  console.log(res2);
}

async function upgrade() {
  const C = await ethers.getContractFactory(NAME);
  const address = await C.attach(ADDR);
  const Factory = await ethers.getContractFactory(NAME);
  console.log(`${NAME} upgrading...`);
  await upgrades.upgradeProxy(address, Factory);
  console.log(`${NAME} upgraded`);
}

async function main() {
  // await upgrade();
  await test();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
