function getCookie(cname) {
  var name = cname + "=";
  var decodedCookie = decodeURIComponent(document.cookie);
  var ca = decodedCookie.split(';');
  for(var i = 0; i <ca.length; i++) {
    var c = ca[i];
    while (c.charAt(0) == ' ') {
      c = c.substring(1);
    }
    if (c.indexOf(name) == 0) {
      return c.substring(name.length, c.length);
    }
  }
  return "";
}
function Disclaimer() {
  var d = new Date();    
  if (
    confirm(
      "By pressing OK you understand that this website could lead you to images showing inappropriate or personnel content."
    )
  ) {
    document.cookie = `dis=1; expires=Thu, 18 Dec ${d.getFullYear() + 10} 12:00:00 UTC`;
  } else {
    document.cookie = `dis=0; expires=Thu, 18 Dec ${d.getFullYear() + 10} 12:00:00 UTC`;
    location.href = 'https://connorcode.com';
  }
}
function stringGen(len) {
  var text = "";
  var charset = "abcdefghijklmnopqrstuvwxyz0123456789";
  for (var i = 0; i < len; i++)
    text += charset.charAt(Math.floor(Math.random() * charset.length));
  return text;
}
function rmg() {
  var x = stringGen(6);
  window.open("https://prnt.sc/" + x.toString());
}
