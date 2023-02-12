(() => {
  const home = document.getElementById("homeButton");
  if (home == null) return;
  home.addEventListener("click", () => {
    document.location = "/";
  });
})();
