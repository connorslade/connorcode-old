const code = document.querySelector('#code');

document.querySelector('#gist112881990').style.display = 'none';

code.addEventListener('click', () => {
  document.querySelector('#gist112881990').style.display = 'block';
});
