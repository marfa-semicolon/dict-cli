function initResultPanel(seeMoreLabel, seeLessLabel) {
    if (document.addEventListener) {
        document.addEventListener("DOMContentLoaded", seeMoreLessLink, false);
    } else if (document.all && !window.opera) {
        document.write('<script type="text/javascript" id="contentloadtag" defer="defer" src="javascript:void(0)"><\/script>');
        var contentloadtag = document.getElementById("contentloadtag");
        contentloadtag.onreadystatechange = function() {
            if (this.readyState == "complete") {
                seeMoreLessLink();
            }
        };
    } else {
        window.onload = seeMoreLessLink;
    }

    //if javascript works add hide class so we can hide the hidden bits
//    $(".z_idsym").bind('click', function(){
//    	$(".id-g").toggleClass('idiom_hide');
//    	$(this).toggleClass('idiom_hide_header');
//    });
    
    $('.accordion dt').toggleClass('hide');
    $('.accordion dt:first-child').toggleClass('hide');

    // add event listener so clicking on h2 will toggle the hide class in the li
    $( ".accordion dt" ).on( "click", function() {
      $(this).siblings( 'dt' ).addClass('hide');
      $(this).removeClass('hide');
    });

    //if javascript works add hide class so we can hide the hidden bits
    $('.open-close dt').toggleClass('hide');

    //add event listener so clicking on h2 will toggle the hide class in the li
    $(".open-close dt").bind("click", function() {
//        console.log("toggle");
        $(this).toggleClass('hide');
    });

    $("a.see-more").on("click", function() {
        var $this = $(this);
        $this.prev().toggleClass('show');
        if ($this.html() == seeMoreLabel) {
            $this.html(seeLessLabel);
        } else {
            $this.html(seeMoreLabel);
        }
        return false;
    });
}

function initWordlistPanel() {
    //add event listener so clicking on h2 will toggle the hide class in the li
    $(".open-close dt").bind("click", function() {
        $(this).toggleClass('hide');
    })
}

function initOpenClose() {

    // add event listener so clicking on h2 will toggle the hide class in the li
    $(".open-close dt").bind("click", function() {
        $(this).siblings('dt').addClass('hide');
       
        if ($(this).hasClass('hide'))
            $(this).removeClass('hide');
        else
            $(this).addClass('hide');
    });
}

function seeMoreLessLink() {
    if (document.getElementById("readmore")) {
        var linkHTMLMore = '<p id="showmore"><a  id="seemore" href="#" onclick="toggleDiv(\'readmore\'); return false;">see more</a></p>';
        var linkHTMLLess = '<p id="showless" style="display:none"><a id="seeless" href="#"  onclick="toggleDiv(\'readmore\'); return false; ">see less</a></p>';
        $('#readmore').before(linkHTMLMore).after(linkHTMLLess);
    }
}

function toggleDiv(target_div){
    try {
        $('#'+target_div).toggle(0);
        $('#showless').toggle(0);
        $('#showmore').toggle(0);
    } catch (err) {}
}

function setActivePronRow(){
    $('.pron_row').on("click", function(){
        $('.pron_row').removeClass('active');
        $(this).addClass('active');
        return false;
    });
    
    $('.pron_button_record').on("click", function(){
      // $(this).removeClass('button_is_active');
      $(this).hide();
      $(this).next().show();
    });
    
    $('.pron_record_stop').on("click", function(){
      $(this).hide();
      $(this).prev().show();
      $(this).next().addClass('button_is_active');
    });
    
    $('.pron_button_play').on("click", function(){
      $(this).hide();
      $(this).next().show();
    });
    
    $('.pron_play_stop').on("click", function(){
      $(this).hide();
      $(this).prev().show();
    });
}
