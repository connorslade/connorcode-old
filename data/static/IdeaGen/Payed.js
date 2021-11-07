// This was all obfuscated but now if you really want to use this shitty program for free then you can
// Your welcome. lol

const locks = [
  "QYR92-1N9LA-SYT34-AWZA0",
  "ZZE5F-LULGZ-Q189B-VRALK",
  "YACNP-FT7QU-16XZZ-7PHL7",
  "6MM2D-OD0MA-SM1TT-URWEZ",
  "ZO75Z-QHZCP-CD1Y0-O5NAI",
  "51NVF-TLYOO-MR7P4-3CBTE",
  "HVGCE-MISCQ-ULFY2-KL4F2",
  "6PAF6-E989D-FU914-9PHP8",
  "SWPHF-HA72C-A158B-DP3LK",
  "IXZ90-G0EZ2-9E8C9-GFQPS",
];

function main() {
  let key = localStorage.getItem("key");

  if (key) {
    document.getElementById("thanks").innerHTML =
      "You have already bought the Entrepreneur Opportunity Generator!";
    document.getElementById("payed").innerHTML = "Your Product Key: " + key;
    return;
  }

  if (!localStorage.getItem("payed")) return;
  let product = locks[Math.floor(Math.random() * locks.length)];
  document.getElementById("thanks").innerHTML =
    "Thank you for Buying the Entrepreneur Opportunity Generator!";
  document.getElementById("payed").innerHTML = "Product Key: " + product;
  localStorage.setItem("key", product);
}

main();
