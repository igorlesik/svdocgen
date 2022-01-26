document.addEventListener("load", loadWaveDrom());

function loadWaveDrom() {
    var script = document.createElement("script");
    script.src = "https://cdnjs.cloudflare.com/ajax/libs/wavedrom/2.6.8/skins/default.js";
    script.onload = function() {
        var script2 = document.createElement("script");
        script2.src = "https://cdnjs.cloudflare.com/ajax/libs/wavedrom/2.6.8/wavedrom.min.js";
        script2.onload = function() { WaveDrom.ProcessAll(); }
        document.head.appendChild(script2);
    }
    document.head.appendChild(script);

    var script3 = document.createElement("script");
    script3.src = "https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js";
    script3.onload = function() { mermaid.initialize({startOnLoad:true}); }
    document.head.appendChild(script3);
}