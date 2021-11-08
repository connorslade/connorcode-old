const code = document.querySelector('#code');

document.querySelector('#gist112881990').style.display = 'none';

code.addEventListener('click', () => {
  document.querySelector('#gist112881990').style.display = 'block';
});

fetch('/pi/pi.txt').then(r => r.text()).then(r => {
  document.querySelector('#pi').innerHTML = r;
})
