document.addEventListener('touchstart', function (event) {
    let element = event.target;
    if (element.dataset.imagePicker) {
        if (window.webkit) {
            let imagePickerResult = element.dataset.imagePickerResult;
            window.webkit.messageHandlers.invokePhotoPicker.postMessage(imagePickerResult);
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
