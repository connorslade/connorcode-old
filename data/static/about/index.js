document.querySelector("#greeting").innerHTML = getGreeting();

function getGreeting() {
  let hour = new Date().getHours();
  if (hour >= 3) return "Good morning";
  if (hour >= 7) return "Good afternoon";
  return "Good evening";
}
  
