pub const HTML_HEADER: &str = r###"
<html><head>
<style>
input {
    outline: none;
    font-size: 1rem;
    padding: 6px 12px;
    margin-bottom: .5rem;
    background: #F0F0F0;
    color: black;
    border: 1px solid transparent;
    border-radius: 4px;
    box-shadow: none;
    box-sizing: border-box;
    height: 40px;
}
input[type="button"] {
    display: inline-block;
    text-align: center;
    text-decoration: none;
    white-space: nowrap;
    background: #0070F3;
    color: white;
    border: 0;
    cursor: pointer;
    width: 40px;
}
input[type="range"] {
    -webkit-appearance: none;
    padding: 0;
    height: 14px;
    width: 600px;
    border-radius: 14px;
}
canvas {
    border: 1px solid black;
}
</style>
<script type="text/javascript">
onload = () => {
    document.getElementById("page-slider").max = page_func.length - 1;
    draw();
};

onkeydown = (e) => {
    e = e || window.event;
    if(e.keyCode == '37') {
        prev_page();
    }
    if(e.keyCode == '39') {
        next_page();
    }
};

var now_page = 0;
function draw() {
    let cv = document.getElementById("c");
    let ctx = cv.getContext("2d");
    ctx.lineWidth=3;
    ctx.fillStyle = "#fff";
    ctx.fillRect(0, 0, 800, 800);
    page_func[now_page](ctx);
    document.getElementById("page-num").innerText = String(now_page + 1) + " / " + String(page_func.length);
    let slider = document.getElementById("page-slider");
    slider.value = now_page;
}

function next_page() {
    if(now_page + 1 < page_func.length) {
        now_page += 1;
    } else {
        stop_play();
    }
    draw();
}

function prev_page() {
    if(0 < now_page) {
        now_page -= 1;
    }
    draw();
}

function jump_page(num) {
    now_page = Math.min(Math.max(0, num), page_func.length);
    draw();
}

var play_interval = 10;
var interval;

function start_play() {
    clearInterval(interval);
    interval = setInterval(next_page, 1000 / play_interval);
    document.getElementById("toggle_play_button").value = "||";
}

function stop_play() {
    clearInterval(interval);
    interval = null;
    document.getElementById("toggle_play_button").value = "▶";
}

function toggle_play() {
    if(interval == null) {
        start_play();
    } else {
        stop_play();
    }
}

function jump_head() {
    stop_play();
    now_page = 0;
    draw();
}

function jump_last() {
    stop_play();
    now_page = page_func.length - 1;
    draw();
}

function save_canvas() {
    let cv = document.getElementById("c");
    let a = document.createElement("a");
    a.href = cv.toDataURL();
    a.download = "result.png";
    a.click();
}

function s(c, color) {
    c.fillStyle = color;
    c.strokeStyle = color;
}

function a(c, x, y, r) {
    c.beginPath();
    c.arc(x, y, r, 0, 6.28);
    c.fill();
}

function l(c, x1, y1, x2, y2) {
    c.beginPath();
    c.moveTo(x1, y1);
    c.lineTo(x2, y2);
    c.closePath();
    c.stroke();
}
"###;

pub const HTML_TAIL: &str = r###"
</script></head><body>
<input type="button" value="|<" onclick="jump_head();"></input>
<input type="button" value="<" onclick="prev_page();"></input>
<input type="button" value="▶" id="toggle_play_button" onclick="toggle_play();"></input>
<input type="button" value=">" onclick="next_page();"></input>
<input type="button" value=">|" onclick="jump_last();"></input>
fps: <input type="number" value="10" oninput="play_interval=this.value;"></input>
<input type="button" value="save png" style="width: 100px;" onclick="save_canvas();"></input>
<br/>
pages: <span id="page-num"></span>
<br/>
<input type="range" id="page-slider" value="0" min="0" step="1" oninput="jump_page(this.value);">
<br/>
<canvas id="c" width="800" height="800"></canvas>
</body></html>
"###;
