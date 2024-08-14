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
      "0x130151e86e8bfc86ee1d7c564e652ca40bcde07f10ed575d7420336faed25bb9",
      "0x186bc1339abffb608382600e4a4c5dc1153a72c17441b0b3517ce761c34650be"
    ],
    [
      [
        //"0x2d1d9d34ed6e720057199ce3f29cc167311d1b2ac754104be7c79db30bb591b0",
        //"0x2056d39820d9a658a83c1dabc0e1b51599e7e01c6a4315923b5017c2edb88cfe"
        "0x22c1bef3725e5ba721ca6423074ffeb5d1ba96695cdbe66b788e4dc37fcf910b",
        "0x09736785156dec28260265bcca0777a00f9ebcb5d7e454b601ce18af70436ffb"
      ],
      [
        //"0x0aa28427983ba6a8493a824911093c411e7073ce41bf890a80815b0f023a9822",
        //"0x2dce498b05b0c0f9758cf4e7da9c30f7f0d966227dfb836e7482220a7a3caa87"
        "0x2b6719a3b93dabe9d45c566be9351bece189cf522ceba95b4fac120044bd8b32",
        "0x295f9cbc10f2ee5ff76b42dbb9ced29bc42a43bb79664d7cb5ef9e4b5a851b50"
      ]
    ],
    [
      //"0x0c04ce9eadbc453c0ff5e7fe30ab185764c054d3f9c50f43187d57d069ac4512",
      //"0x082103fa2ad8ee293a52dafbb1c0a47d42eafb0d13400efe27d31b980bf7a3f9"
      "0x2aa6d54873116b952dfdfa61f1d0afc14cd86b8cd02cdcd0e798a190bb63ab97",
      "0x103be5edaec43213fc4e1599cb5ece895ac425655ed4ba3e3d819b666dc87562"
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
