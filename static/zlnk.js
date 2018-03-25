$(document).ready(() => {
    $('#form').submit((e) => {
        e.preventDefault();
        const url = $('#longURL').val();
        $('#longURLFeedback').text('');
        $('#longURL').removeClass('is-invalid');
        $('#longURL').prop('disabled', true);
        $('#submitButton').prop('disabled', true);
        $.post({url: '/shorten', data: url, contentType: 'text/plain'}).done((res) => {
            const url = window.location.href + res;
            $('#result').html('<a href="' + url + '">' + url + '</a>');
            $('#resultContainer').removeClass('invisible');
        }).fail((error) => {
            $('#longURL').addClass('is-invalid');
            $('#longURLFeedback').text(error.responseText);
        }).always(() => {
            $('#longURL').prop('disabled', false);
            $('#submitButton').prop('disabled', false);
        })
    })
})