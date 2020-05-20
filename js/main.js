let pesterlogBtn = document.getElementById('open-pesterlog');
let pesterlog = document.getElementById('pesterlog');

if (pesterlogBtn !== null && pesterlog !== null) {
    let typ = pesterlogBtn.getAttribute('data-typ');
    let shown = false;
    pesterlogBtn.addEventListener('click', () => {
        shown = !shown;
        if (shown) {
            pesterlogBtn.innerHTML = `Hide ${typ}`;
            pesterlog.classList.add('opened');
        } else {
            pesterlogBtn.innerHTML = `Show ${typ}`;
            pesterlog.classList.remove('opened');
        }
    });
}

let next = document.querySelector('.next');
let back = document.getElementById('back');

document.addEventListener("keydown", event => {
    if (event.isComposing || event.keyCode === 229) {
        return;
    }

    let target = event.target;
    if (target.tagName.toLowerCase() == 'input') return;

    if (back !== null && event.key === 'ArrowLeft') {
        back.click();
    } else if (next !== null && event.key === 'ArrowRight') {
        next.click();
    }
});

const content = document.getElementById('content');

if (content !== null) {
    for (const radio of document.getElementsByName('img-size')) {
        const size = parseInt(radio.getAttribute('data-size'));
        const label = (size / 10) * window.devicePixelRatio;
        radio.labels[0].textContent = `${label}x`;
        radio.addEventListener('click', () => {
            localStorage.setItem('img-size', size);
            content.className = `size-${size}`;
        });
    }
}

let defaultSize = localStorage.getItem('img-size');
if (defaultSize === null) {
    if (window.devicePixelRatio === 2) {
        defaultSize = '15';
    } else {
        defaultSize = '10';
    }
}

document.getElementById(`size-${defaultSize}-radio`).click();
