function onVideoClick(theLink) {
    var string = navigator.userAgent;
    if (string.search("Edge") >= 0){
        if (confirm("Since you are using Edge you will be redirected to the video. If the video is not playing click ok. Just use Firefox")) {
            window.location = theLink;
        }
    }else if (string.search("iPhone") >= 0){
        if (confirm("Since you are on an iPhone you will be redirected to the video.")){
            window.location = theLink;
        }
    }else if (string.search("Trident") >= 0){
        if (confirm("Since you are on using internet explorer you may need to download the video to view it. Internet explorer is a 25 year old web browser, Just use Firefox.")){
            window.location = theLink;
        }
    }else {
        document.getElementById("video_pop").innerHTML = "<video autoplay unmuted controls playsinline id=\"the_Video\" class=\"fadeIn\"><source src=\""+theLink+"\" type=\"video/webm\"></video>";
        document.getElementById("video_pop").style.display="block";
    }
} 

function onPopClick() {
    document.getElementById("video_pop").style.display="none";
    document.getElementById("video_pop").innerHTML = ""; 
}         