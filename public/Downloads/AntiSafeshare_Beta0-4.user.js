// ==UserScript==
// @name         Anti Safeshare
// @namespace    https://connorcode.com
// @version      Beta_0.5
// @description  Redirects to the orignal youtube video!
// @author       Sigma76 (Connor Slade)
// @update       https://connorcode.com/Downloads/AntiSafeshare_Beta0-4.user.js
// @match        https://safeshare.tv/*/*
// @match        https://video.link/*/*
// ==/UserScript==

(function () {
  "use strict";
  if (window.location.href.includes("safeshare.tv") == true) {
    redirectSS();
  } else if (window.location.href.includes("video.link") == true) {
    redirectVL();
  }
  function redirectSS() {
    try {
      var id = document
        .getElementById("iframe-embed")
        .attributes[2].nodeValue.split("/")[4]
        .split("?")[0];
      window.location.replace("https://vimeo.com/" + id);
    } catch (err) {
      var id = document
        .getElementById("iframe-embed")
        .src.split("videoID=")[1]
        .split("&")[0];
      window.location.replace("https://youtube.com/watch?v=" + id);
    }
  }
  function redirectVL() {
    var id = document.getElementsByTagName("script");
    for (var i in id) {
      if (String(id[i].innerHTML).includes("var safeYT")) {
        var index = i;
      }
    }
    id = document
      .getElementsByTagName("script")
      [index].innerHTML.split(",")[1]
      .split("'")[1];
    window.location.replace("https://youtube.com/watch?v=" + id);
  }
})();
