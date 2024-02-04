(function(exports) {
    var copy = function (text) {
        const textarea = document.createElement("textarea");
        textarea.value = text;
        textarea.style.position = "fixed";
        textarea.style.opacity = "0";
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand("copy");
        document.body.removeChild(textarea);
    };

    class JQuery {
        constructor(selector) {
            if (typeof selector === 'string') {
                this.selector = selector
                this.elements = document.querySelectorAll(selector)
                if (this.elements.length < 1) {
                    throw new Error('selector empty')
                }
            } else if (Array.isArray(selector)) {
                this.elements = selector
            } else if (selector instanceof HTMLElement) {
                this.elements = [selector]
            } else if (selector instanceof HTMLCollection) {
                this.elements = selector
            } else {
                throw new Error('invalid selector type')
            }
        }
        each(fn) {
            for (var i = 0; i < this.elements.length; i++) {
                fn(i, this.elements[i])
            }
            return this
        }
        bind(type, fn) {
            this.each(function(i, el) {
                el.addEventListener(type, fn)
            })
            return this
        }
        size() {
            return this.elements.length
        }

        dom() {
            return this.elements[0]
        }

        parent() {
            return $(this.elements[0].parentElement)
        }
        children() {
            return $(this.elements[0].children)
        }

        remove() {
            this.each(function(i, el) {
                el.remove()
            })
            return this
        }

        addClass(className) {
            this.each(function(i, el) {
                el.classList.add(className)
            })
            return this
        }
        removeClass(className) {
            this.each(function(i, el) {
                el.classList.remove(className)
            })
            return this
        }
        toggleClass(className, force) {
            this.each(function(i, el) {
                el.classList.toggle(className, force)
            })
            return this
        }
        show() {
            this.each(function(i, el) {
                el.style.display = ''
            })
            return this
        }
        hide() {
            this.each(function(i, el) {
                el.style.display = 'none'
            })
            return this
        }

        value(val) {
            if (val === undefined) {
                return this.elements[0].value
            } else {
                this.elements[0].value = val
                return this
            }
        }
        html(html) {
            if (html === undefined) {
                return this.elements[0].innerHTML
            } else {
                this.elements[0].innerHTML = html
                return this
            }
        }
        text(text) {
            if (text === undefined) {
                return this.elements[0].innerText
            } else {
                this.elements[0].innerText = text
                return this
            }
        }
    }

    // constructor
    var $ = function(selector) {
        return new JQuery(selector)
    }
    // static method
    $.create = function(tagName, props) {
        var el = document.createElement(tagName)
        if (props) {
            for (var key in props) {
                if (!props.hasOwnProperty(key)) continue
                if (key === 'style') {
                    Object.assign(el.style, props[key])
                } else {
                    el[key] = props[key]
                }
            }
        }
        return $(el)
    }
    // static document method alias
    $.qcs = document.queryCommandState.bind(document)
    $.ec = document.execCommand.bind(document)


    var request = function(method, url, data, callback) {
        var headers = {'Content-Type': 'application/json'}
        var body = null

        if (method === "POST") {
            headers["Content-Type"] = "application/x-www-form-urlencoded"

            var params = []
            for (var key in data) {
                params.push(key + "=" + encodeURIComponent(data[key]))
            }
            body = params.join("&")
        }
        fetch(url, {
            method: method,
            headers: headers,
            body: body,
            credentials: 'include'
        }).then(function(response) {
            return response.json()
        }).then(function(json) {
            callback(json, null)
        }).catch(function(ex) {
            callback(null, ex)
            console.error('err occurred: ', ex)
        })
    }
    var doGet = function(url, callback) {
        return request("GET", url, null, callback)
    }
    var doPost = function(url, data, callback) {
        return request("POST", url, data, callback)
    }

    $.get = doGet
    $.post = doPost
    $.copy = copy;

    exports.$ = $;
})(window);

