// Automaticly update Github Lat Update Date :P
let xmlHttp = new XMLHttpRequest();
xmlHttp.open("GET", "https://api.github.com/users/basicprogrammer10", false);
xmlHttp.send(null);
let lastUpdate = JSON.parse(xmlHttp.responseText)['updated_at'].split('-')
let data = `${lastUpdate[1]}/${lastUpdate[2].split('T')[0]}/${lastUpdate[0]}`

document.getElementById('gh').innerHTML = data