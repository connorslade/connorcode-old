document.querySelector("#greeting").innerHTML = getGreeting();
document.querySelector("#email").innerHTML =
  '<a href="mailto:connor@connorcode.com">connor@connorcode.com</a>';

function getGreeting() {
  let hour = new Date().getHours();
  if (hour >= 17) return "Good evening";
  if (hour >= 12) return "Good afternoon";
  if (hour >= 3) return "Good morning";
  return "Good evening";
}

feather.replace();
