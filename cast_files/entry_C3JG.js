function initEntry(seeMoreLabel, seeLessLabel) {
    initResultPanel(seeMoreLabel, seeLessLabel);
    $('a.topic').lightBox({imageBtnClose: lightboxImageBtnClose});
    $(".unbox .box_title, .unbox .heading").on("click", function(){
        $(this).parent().toggleClass("is-active");
    });
}
