import 'morphdom/dist/morphdom-umd.js';
import "jquery";
import "popper.js";
import "bootstrap";
import Typed from "typed.js";
import fromEntries from 'fromentries';


document.addEventListener('touchstart', function (event) {
    let element = event.target;
    if (element.dataset.imagePicker) {
        if (window.webkit) {
            let imagePickerResult = element.dataset.imagePickerResult;
            window.webkit.messageHandlers.invokePhotoPicker.postMessage(imagePickerResult);
        }

        if (window.Android) {
            let imagePickerResult = element.dataset.imagePickerResult;

            window.Android.invokePhotoPicker(imagePickerResult);
        }
    }
});

window.positivelys = {};
window.positivelys.setImagePickerPath = function (path, inputId) {
    let input = document.getElementById(inputId);

    input.value = path;

    let imagePickerPreview = document.getElementById('image-picker-preview');
    imagePickerPreview.src = path
    imagePickerPreview.parentElement.classList.remove('d-none');
}

let noPositivelys = document.getElementById('noPositivelys');
let whatsPositive = document.getElementById('whatsPositive');

if (noPositivelys && whatsPositive) {
    var options = {
        strings: ['', 'Got coffee with a friend ☕', 'I ate a healthy 🥗 lunch', "Ran a mile! 🏃👟🎉️", "What's positive about today?"],
        typeSpeed: 70,
        startDelay: 6000
    };

    var typed = new Typed('#whatsPositive', options);
}

async function makeWebAppRequest(url, method, bodyAsJson) {
    let response;

    if (method.toLowerCase() === "get") {
        response = await fetch(url);
    } else if (method.toLowerCase() === "post") {
        response = await fetch(url, {
            method: 'POST',
            redirect: 'follow',
            body: bodyAsJson
        });
    }

    const htmlResponse = await response.text();
    morphdom(document.documentElement, htmlResponse);
    history.pushState(undefined, undefined, response.url);
}

async function makeAppRequest(uri, method = 'GET', body = undefined) {
    if (window.webkit) {
        window.webkit.messageHandlers.makeAppRequest.postMessage(JSON.stringify({
            uri,
            method,
            body
        }));
    } else if (window.Android) {
        window.Android.makeAppRequest(JSON.stringify({
            uri,
            method,
            body
        }));
    } else {
        await makeWebAppRequest(uri, method, body)
    }
}

document.addEventListener('submit', async function (event) {
    event.preventDefault();

    const formData = new FormData(event.target);
    const data = fromEntries(formData.entries());
    await makeAppRequest(event.target.action, event.target.method.toUpperCase(), JSON.stringify(data))
});

document.addEventListener('click', async function (event) {
    let anchorTag = event.target;
    let childElementOfAnchorClick = event.target.closest('a');
    if (anchorTag.tagName.toLowerCase() === "a") {
        event.preventDefault();

        await makeAppRequest(anchorTag.href)
    } else if (childElementOfAnchorClick) {
        event.preventDefault();

        await makeAppRequest(childElementOfAnchorClick.href)
    }
});

window.addEventListener('popstate', async function (event) {
    let url = event.target.location.toString();

    await makeAppRequest(url)
});