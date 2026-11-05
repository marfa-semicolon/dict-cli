(function($){
    // add word list star
    $(".sn-gs .sn-g, .senses_multiple .sense, .sense_single .sense").each(function() {
        $(this).prepend(
            '<a class="open oup_icons" title="' + TRANSLATIONS["mywordlist.add.header"] + '">' +
                '<span class="star-btn" aria-hidden="true">&#8203;</span>' +
            '</a>');
    }); 

    // word list star click
    $(".star-btn").on("click", function(e) {
        if (wordlistService.isPopupDisplayed())
            wordlistService.closePopup();
        else {
            var currentNode = $(this);
            var successCb = function() {
                if(!authorized)
                    window.location.href = window.oldoApi._baseUrl + "mywordlist/";
                else
                    wordlistService.openPopup(currentNode);
            }
            var failureCb = function(response, requestOptions) {
                if (response.status == 404)
                    window.location.href = window.oldoApi._baseUrl + "mywordlist/";
                else
                    alert(getErrorMessage(response, requestOptions, ""));
            }
            window.oldoApi.getCurrentUser(successCb, failureCb);
        }
        e.stopPropagation();
    });

    $("body").click(function() {
        if (wordlistService.isPopupDisplayed())
            wordlistService.closePopup();
    });

    $(window).resize(function() {
        wordlistService.refreshPopupPosition();
    });

    $(window).scroll(function() {
        wordlistService.refreshPopupPosition();
    });

})(jQuery);

function getErrorMessage(response, requestOptions, errorMessage) {
    var responseJSON = response.responseJSON;
    if (responseJSON && responseJSON.errorMessage)
        errorMessage += responseJSON.errorMessage;
    else if (requestOptions.errorMessage)
        errorMessage += requestOptions.errorMessage;
    else if (errorMessage === "")
        errorMessage = "An error occurred";
    return errorMessage;
}

var wordlistService = {

    button: null,
    popup: null,

    addEntry: function(spanCurrent) {
        var wordlistId = spanCurrent.parent().attr("data-id");
        var wordlistName = spanCurrent.parent().attr("data-name");
        var wordlistEntry = {
            "dictCode" : pageDictCode,
            "entryId" : spanCurrent.closest("div.entry")[0].id.trim(),
            "senseId" : spanCurrent.closest(".sense")[0].id.trim()
        };
        var successCb = function() {
            var infoMsg = TRANSLATIONS["mywordlist.add.senseAdded"] + " '" + escapeHtml(wordlistName) + "'";
            wordlistService.appendMessagePopup("info", infoMsg);
        };
        var failureCb = function(response, requestOptions) {
            if (response.status == 403)
                window.location.replace(window.oldoApi._baseUrl + "mywordlist/");
            else {
                var errorMsg = getErrorMessage(response, requestOptions, TRANSLATIONS["wordlist.error.info"] + " ");
                wordlistService.appendMessagePopup("error", errorMsg);
            }
        };
        window.oldoApi.createWordlistEntry(wordlistId, wordlistEntry, successCb, failureCb);
    },

    appendMessagePopup : function(type, message) {
        var popup = wordlistService.popup;
        var div = "<div class='message " + type + "' style='display:none;'></div>";
        var messagePanel = popup.find(".message");
        if (messagePanel.length > 0) {
            popup.find(".message").slideUp(200, function() {
                $(this).remove();
            });
        }
        var messagePanel = $(div);
        popup.append(messagePanel);
        messagePanel.html(message);
        messagePanel.slideDown(200);

        setTimeout(function() {
            messagePanel.slideUp(200, function() {
                $(this).remove();
            });
        }, 4000);
    },

    closePopup : function() {
        var self = this;
        this.popup.fadeOut(200, function() {
            self.popup.remove();
            self.popup = null;
            self.button.removeClass("current");
            self.button = null;
        });
    },

    isPopupDisplayed : function() {
        return this.popup !== null && this.button !== null && ($(".footer").length > 0 || $(".spinner").length > 0)
    },

    openPopup : function(btn) {
        // snipper
        this.button = btn;
        this.button.addClass("current");
        this.popup = $("<div class='wordlist-popup' style='display:none;'><img class='spinner' src='" + URLS["spinner_small"] + "'/></div>");
        this.refreshPopupPosition();

        this.button.append(this.popup);
        this.popup.fadeIn(200);
        this.popup.refresh;

        // words lists
        var wordlistDictCode = pageDictCode;
        this.wordlistsGet(this.popup);

        // create words list and entry
        this.popup.on("click", ".wordlist-submit", function() {
            var newWordListName = $(this).parent().find("input[type='text']").val().trim();
            var wordlist = {
                "name" : newWordListName
            };
            var successCb = function(newList) {
                var infoMsg = TRANSLATIONS["mywordlist.add.wordlistAdded"].replace("{0}", escapeHtml(newList.name));
                wordlistService.appendMessagePopup("info", infoMsg);
                wordlistService.popup.refresh;
                // refresh the list and add the entry
                wordlistService.wordlistsRefreshAndAddEntry(wordlistService.popup, newList.id);
            };
            var failureCb = function(response, requestOptions) {
                var errorMsg = getErrorMessage(response, requestOptions, TRANSLATIONS["mywordlist.add.error.unexpected"] + " ");
                wordlistService.appendMessagePopup("error", errorMsg);
            };
            window.oldoApi.createWordlist(wordlist, successCb, failureCb);
        });

        // close the popup
        this.popup.on("click", ".popup-close", function() {
            wordlistService.closePopup();
        });

        // add word to list
        this.popup.on("click", ".plus-btn", function() {
            wordlistService.addEntry($(this));
        });

        // add
        this.popup.click(function(e) {
            e.stopPropagation();
        });
    },

    refreshPopupPosition : function() {
        if (this.button != null && this.button.length > 0 && this.popup != null && this.popup.length > 0) {
            var curPos = this.button.offset().top - $(window).scrollTop();
            var curLeft = this.button.offset().left;
            var screenHeight = window.innerHeight;
            var entryDiv = document.getElementsByClassName('responsive_entry_center_wrap')
            if (entryDiv && entryDiv[0])
                var entryDivTop = entryDiv[0].offsetTop

            if (curPos < screenHeight / 2 || (entryDivTop && curPos-entryDivTop < 300))
                this.popup.addClass("under");
            else
                this.popup.removeClass("under");

            if (curLeft < 500)
                this.popup.addClass("right");
            else
                this.popup.removeClass("right");
        }
    },

    refreshWordlistPopup : function(popup, userWordlists) {
        // get words lists
        var userLists = userWordlists.userLists;
        var wordlists = "";
        for (var i = 0; i < userLists.length; i++) {
            var d = userLists[i];
            wordlists += "" +
                "<li class='clear-after'>" +
                    "<a href='" + URLS["wordlist"] + d.id + "' class='wordlistname' title='" + TRANSLATIONS["wordlist.popup.link"] + "'>" +
                        escapeHtml(d.name) + 
                    "</a>" +
                    "<a data-id='" + d.id + "' data-name='" + escapeHtml(d.name) + "' class='add oup_icons' title='" + TRANSLATIONS["wordlist.popup.add"] + "'>" +
                        "<div class='plus-btn'></div>" +
                    "</a>" +
                "</li>";
        }

        // format HTML
        var html = "" +
            "<div class='header' title=''>" + "<div class='header_title'>" + TRANSLATIONS["wordlist.popup.add"] + "</div>" + 
                "<div class='popup-close' title='" + TRANSLATIONS["wordlist.popup.close"] + "'>" + TRANSLATIONS["wordlist.popup.close"] + "&nbsp;<span class='cross-btn'></span></div>" +
            "</div>" + 
            "<ul>" +
                wordlists + 
            "</ul>" +
            "<div class='footer' title=''>" +
                TRANSLATIONS["wordlist.popup.create"] +
                "<div class='wordlist-form'>" +
                    "<div class='wordlist-submit'>" + TRANSLATIONS["wordlist.popup.submit"] + "</div>" +
                    "<div style='overflow:hidden;'> <input type='text' placeholder='" + TRANSLATIONS["wordlist.popup.input"] + "' /></div>" +
                "</div>"+
            "</div>";

        popup.html(html);
    },

    wordlistsGet : function(popup) {
        var successCb = function(userWordlists) {
            wordlistService.refreshWordlistPopup(popup, userWordlists);
        };
        var failureCb = function(response, requestOptions) {
            var errorMsg = getErrorMessage(response, requestOptions, TRANSLATIONS["wordlist.error.info"] + " ");
            wordlistService.appendMessagePopup("error", errorMsg);
        };
        window.oldoApi.getWordlists(successCb, failureCb);
    },

    wordlistsRefreshAndAddEntry : function(popup, listId) {
        var successCb = function(userWordlists) {
            wordlistService.refreshWordlistPopup(popup, userWordlists);
            wordlistService.addEntry(popup.find("a[data-id='" + listId + "']").find("div[class='plus-btn']"));
        };
        var failureCb = function(response, requestOptions) {
            var errorMsg = getErrorMessage(response, requestOptions, TRANSLATIONS["wordlist.error.info"] + " ");
            wordlistService.appendMessagePopup("error", errorMsg);
        };
        window.oldoApi.getWordlists(successCb, failureCb);
    }
};