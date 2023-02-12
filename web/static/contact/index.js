// Define Contact Data
const data = {
  website: [
    [
      "http://zgurppzm423u2ny7ljdj2m33na4wxoo7zvkvvu4vpce2ure5gplexnqd.onion",
      "http://zgurppzm423u2ny7ljdj2m33na4wxoo7zvkvvu4vpce2ure5gplexnqd.onion",
    ],
    ["https://connorcode.com", "https://connorcode.com"],
  ],
  github:  [["https://github.connorcode.com", "https://github.connorcode.com"]],
  discord: [["Sigma#8214"]],
  email:   [["connor@connorcode.com", "mailto:connor@connorcode.com"]],
  gpg_key: [["https://connorcode.com/key", "https://connorcode.com/key"]],
};

// Remove 'this site requires js' message
document.getElementById("js").remove();

// Create Links to click for info
Object.keys(data).forEach((key) => {
  let element = document.createElement("div");
  element.innerHTML = `<p>✦ </p><a href="#" class="items" id="${key}">[${key
    .replace(/_/g, " ")
    .toUpperCase()}]</a>`;
  element.addEventListener("click", () => {
    showData(key, element);
  });
  document.body.appendChild(element);
});

// On link click show said Info
function showData(id, node) {
  if (!(id in data)) return;
  let userData = data[id];
  delete data[id];
  console.log(userData);
  userData.forEach((i) => {
    let element = document.createElement("P");
    element.classList.add("infoValue");
    element.innerHTML = `➥ ${i[0]}<br>`;
    if (i[1] != undefined)
      element.innerHTML = `➥ <a href="${i[1]}" class="userDataLink">${i[0]}</a><br>`;
    node.parentNode.insertBefore(element, node.nextSibling);
  });
}
