fetch('/pi/pi.txt').then(r => r.text()).then(r => {
  document.querySelector('#pi').innerHTML = r;
})
