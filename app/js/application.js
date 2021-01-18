document.addEventListener('touchstart', function (event) {
    let element = event.target;
console.log('here');
    if (element.dataset.imagePicker) {
        if (window.webkit) {
            console.log('her2e');
            let imagePickerResult = element.dataset.imagePickerResult;
            console.log('imagePickerResult:', imagePickerResult);
            window.webkit.messageHandlers.invokePhotoPicker.postMessage(imagePickerResult);
            event.preventDefault();
        }
    }
});

window.positivelys = {};
window.positivelys.setImagePickerPath = function (path, inputId) {
    let input = document.getElementById(inputId);

    input.value = path;

    let htmlImageElement = document.createElement('img');
    htmlImageElement.src = path
    document.body.appendChild(htmlImageElement);
}