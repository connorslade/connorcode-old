// Define Contact Data
const data = {
  'github' : ['https://github.connorcode.com', 'https://github.connorcode.com'],
  'discord': ['Sigma#8214'],
  'email'  : ['connor@connorcode.com', 'mailto:connor@connorcode.com']
}

// Remove 'this site requires js' message
document.getElementById('js').remove()

// Create Links to click for info
Object.keys(data).forEach(key => {
  let element = document.createElement('div')
  element.innerHTML = `<p>✦ </p><a href="#" class="items" id="${key}">[${key.toUpperCase()}]</a>`
  element.addEventListener("click", () => { showData(key, element) })
  document.body.appendChild(element)
});

// On link click show said Info
function showData(id, node) {
  if (!(id in data)) return;
  let userData = data[id]
  delete data[id]
  let element = document.createElement("P")
  element.classList.add("infoValue");
  element.innerHTML = `➥ ${userData[0]}`
  if (userData[1] != undefined) element.innerHTML = `➥ <a href="${userData[1]}" class="userDataLink">${userData[0]}</a>`
  node.parentNode.insertBefore(element, node.nextSibling);
}
