function encWbi(e, t, k) {
    t || (t = {});
    //{"imgKey":"7cd084941338484aae1ad9425b84077c","subKey":"4932caff0ff746eab6f01bf08b70ac45"}'
    var r = {
        imgKey: k.imgKey,
        subKey: k.subKey
    }
        , n = r.imgKey
        , i = r.subKey;
    if (n && i) {
        var o = getMixinKey(n + i)
            , a = Math.round(Date.now() / 1e3)
            , s = {
            wts: a
        };
        t.addSelf && (s.w_ks = swapString("".concat(n).concat(i), 2));
        for (var l = Object.assign({}, e, s), c = Object.keys(l).sort(), u = [], d = /[!'\(\)*]/g, p = 0; p < c.length; p++) {
            var h = c[p]
                , f = l[h];
            f && "string" == typeof f && (f = f.replace(d, "")),
            null != f && u.push("".concat(encodeURIComponent(h), "=").concat(encodeURIComponent(f)))
        }
        var m = u.join("&")
            , v = {
            w_rid: md5(m + o),
            wts: a.toString()
        };
        return s.w_ks && (v.w_ks = s.w_ks),
            v
    }
    return null
}

function createCommonjsModule(e, t) {
    return e(t = {
        exports: {}
    }, t.exports),
        t.exports
}

function wordsToBytes(e) {
    for (var t = [], r = 0; r < 32 * e.length; r += 8)
        t.push(e[r >>> 5] >>> 24 - r % 32 & 255);
    return t
}

function bytesToString(e) {
    for (var t = [], r = 0; r < e.length; r++)
        t.push(String.fromCharCode(e[r]));
    return t.join("")
}

function endian(e) {
    if (e.constructor == Number)
        return 16711935 & r.rotl(e, 8) | 4278255360 & r.rotl(e, 24);
    for (var t = 0; t < e.length; t++)
        e[t] = endian(e[t]);
    return e
}

function stringToBytes(e) {
    for (var t = [], r = 0; r < e.length; r++)
        t.push(255 & e.charCodeAt(r));
    return t
}

function bytesToWords(e) {
    for (var t = [], r = 0, n = 0; r < e.length; r++,
        n += 8)
        t[n >>> 5] |= e[r] << 24 - n % 32;
    return t
}
r = {
    rotl: function(e, t) {
        return e << t | e >>> 32 - t
    },
    rotr: function(e, t) {
        return e << 32 - t | e >>> t
    },
    endian: function(e) {
        if (e.constructor == Number)
            return 16711935 & r.rotl(e, 8) | 4278255360 & r.rotl(e, 24);
        for (var t = 0; t < e.length; t++)
            e[t] = r.endian(e[t]);
        return e
    },
    randomBytes: function(e) {
        for (var t = []; e > 0; e--)
            t.push(Math.floor(256 * Math.random()));
        return t
    },
    bytesToWords: function(e) {
        for (var t = [], r = 0, n = 0; r < e.length; r++,
            n += 8)
            t[n >>> 5] |= e[r] << 24 - n % 32;
        return t
    },
    wordsToBytes: function(e) {
        for (var t = [], r = 0; r < 32 * e.length; r += 8)
            t.push(e[r >>> 5] >>> 24 - r % 32 & 255);
        return t
    },
    bytesToHex: function(e) {
        for (var t = [], r = 0; r < e.length; r++)
            t.push((e[r] >>> 4).toString(16)),
                t.push((15 & e[r]).toString(16));
        return t.join("")
    },
    hexToBytes: function(e) {
        for (var t = [], r = 0; r < e.length; r += 2)
            t.push(parseInt(e.substr(r, 2), 16));
        return t
    },
    bytesToBase64: function(e) {
        for (var r = [], n = 0; n < e.length; n += 3)
            for (var i = e[n] << 16 | e[n + 1] << 8 | e[n + 2], o = 0; o < 4; o++)
                8 * n + 6 * o <= 8 * e.length ? r.push(t.charAt(i >>> 6 * (3 - o) & 63)) : r.push("=");
        return r.join("")
    },
    base64ToBytes: function(e) {
        e = e.replace(/[^A-Z0-9+\/]/gi, "");
        for (var r = [], n = 0, i = 0; n < e.length; i = ++n % 4)
            0 != i && r.push((t.indexOf(e.charAt(n - 1)) & Math.pow(2, -2 * i + 8) - 1) << 2 * i | t.indexOf(e.charAt(n)) >>> 6 - 2 * i);
        return r
    }
}
var crypt = createCommonjsModule((function(e) {
    var t, r;
    t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        r = {
            rotl: function(e, t) {
                return e << t | e >>> 32 - t
            },
            rotr: function(e, t) {
                return e << 32 - t | e >>> t
            },
            endian: function(e) {
                if (e.constructor == Number)
                    return 16711935 & r.rotl(e, 8) | 4278255360 & r.rotl(e, 24);
                for (var t = 0; t < e.length; t++)
                    e[t] = r.endian(e[t]);
                return e
            },
            randomBytes: function(e) {
                for (var t = []; e > 0; e--)
                    t.push(Math.floor(256 * Math.random()));
                return t
            },
            bytesToWords: function(e) {
                for (var t = [], r = 0, n = 0; r < e.length; r++,
                    n += 8)
                    t[n >>> 5] |= e[r] << 24 - n % 32;
                return t
            },
            wordsToBytes: function(e) {
                for (var t = [], r = 0; r < 32 * e.length; r += 8)
                    t.push(e[r >>> 5] >>> 24 - r % 32 & 255);
                return t
            },
            bytesToHex: function(e) {
                for (var t = [], r = 0; r < e.length; r++)
                    t.push((e[r] >>> 4).toString(16)),
                        t.push((15 & e[r]).toString(16));
                return t.join("")
            },
            hexToBytes: function(e) {
                for (var t = [], r = 0; r < e.length; r += 2)
                    t.push(parseInt(e.substr(r, 2), 16));
                return t
            },
            bytesToBase64: function(e) {
                for (var r = [], n = 0; n < e.length; n += 3)
                    for (var i = e[n] << 16 | e[n + 1] << 8 | e[n + 2], o = 0; o < 4; o++)
                        8 * n + 6 * o <= 8 * e.length ? r.push(t.charAt(i >>> 6 * (3 - o) & 63)) : r.push("=");
                return r.join("")
            },
            base64ToBytes: function(e) {
                e = e.replace(/[^A-Z0-9+\/]/gi, "");
                for (var r = [], n = 0, i = 0; n < e.length; i = ++n % 4)
                    0 != i && r.push((t.indexOf(e.charAt(n - 1)) & Math.pow(2, -2 * i + 8) - 1) << 2 * i | t.indexOf(e.charAt(n)) >>> 6 - 2 * i);
                return r
            }
        },
        e.exports = r
}    ))

isBuffer_1 = function(e) {
    return null != e && (isBuffer(e) || isSlowBuffer(e) || !!e._isBuffer)
}

function isSlowBuffer(e) {
    return "function" == typeof e.readFloatLE && "function" == typeof e.slice && isBuffer(e.slice(0, 0))
}
function isBuffer(e) {
    return !!e.constructor && "function" == typeof e.constructor.isBuffer && e.constructor.isBuffer(e)
}

var md5 = createCommonjsModule((function(e) {
    var t, r, n, i, o;
    t = crypt,
        // r = charenc_1.utf8,
        n = isBuffer_1,
        // i = charenc_1.bin,
        (o = function e(o, a) {
                o.constructor == String ? o = a && "binary" === a.encoding ? stringToBytes(o) : stringToBytes(o) : n(o) ? o = Array.prototype.slice.call(o, 0) : Array.isArray(o) || o.constructor === Uint8Array || (o = o.toString());
                for (var s = bytesToWords(o), l = 8 * o.length, c = 1732584193, u = -271733879, d = -1732584194, p = 271733878, h = 0; h < s.length; h++)
                    s[h] = 16711935 & (s[h] << 8 | s[h] >>> 24) | 4278255360 & (s[h] << 24 | s[h] >>> 8);
                s[l >>> 5] |= 128 << l % 32,
                    s[14 + (l + 64 >>> 9 << 4)] = l;
                var f = e._ff
                    , m = e._gg
                    , v = e._hh
                    , g = e._ii;
                for (h = 0; h < s.length; h += 16) {
                    var y = c
                        , _ = u
                        , b = d
                        , $ = p;
                    c = f(c, u, d, p, s[h + 0], 7, -680876936),
                        p = f(p, c, u, d, s[h + 1], 12, -389564586),
                        d = f(d, p, c, u, s[h + 2], 17, 606105819),
                        u = f(u, d, p, c, s[h + 3], 22, -1044525330),
                        c = f(c, u, d, p, s[h + 4], 7, -176418897),
                        p = f(p, c, u, d, s[h + 5], 12, 1200080426),
                        d = f(d, p, c, u, s[h + 6], 17, -1473231341),
                        u = f(u, d, p, c, s[h + 7], 22, -45705983),
                        c = f(c, u, d, p, s[h + 8], 7, 1770035416),
                        p = f(p, c, u, d, s[h + 9], 12, -1958414417),
                        d = f(d, p, c, u, s[h + 10], 17, -42063),
                        u = f(u, d, p, c, s[h + 11], 22, -1990404162),
                        c = f(c, u, d, p, s[h + 12], 7, 1804603682),
                        p = f(p, c, u, d, s[h + 13], 12, -40341101),
                        d = f(d, p, c, u, s[h + 14], 17, -1502002290),
                        c = m(c, u = f(u, d, p, c, s[h + 15], 22, 1236535329), d, p, s[h + 1], 5, -165796510),
                        p = m(p, c, u, d, s[h + 6], 9, -1069501632),
                        d = m(d, p, c, u, s[h + 11], 14, 643717713),
                        u = m(u, d, p, c, s[h + 0], 20, -373897302),
                        c = m(c, u, d, p, s[h + 5], 5, -701558691),
                        p = m(p, c, u, d, s[h + 10], 9, 38016083),
                        d = m(d, p, c, u, s[h + 15], 14, -660478335),
                        u = m(u, d, p, c, s[h + 4], 20, -405537848),
                        c = m(c, u, d, p, s[h + 9], 5, 568446438),
                        p = m(p, c, u, d, s[h + 14], 9, -1019803690),
                        d = m(d, p, c, u, s[h + 3], 14, -187363961),
                        u = m(u, d, p, c, s[h + 8], 20, 1163531501),
                        c = m(c, u, d, p, s[h + 13], 5, -1444681467),
                        p = m(p, c, u, d, s[h + 2], 9, -51403784),
                        d = m(d, p, c, u, s[h + 7], 14, 1735328473),
                        c = v(c, u = m(u, d, p, c, s[h + 12], 20, -1926607734), d, p, s[h + 5], 4, -378558),
                        p = v(p, c, u, d, s[h + 8], 11, -2022574463),
                        d = v(d, p, c, u, s[h + 11], 16, 1839030562),
                        u = v(u, d, p, c, s[h + 14], 23, -35309556),
                        c = v(c, u, d, p, s[h + 1], 4, -1530992060),
                        p = v(p, c, u, d, s[h + 4], 11, 1272893353),
                        d = v(d, p, c, u, s[h + 7], 16, -155497632),
                        u = v(u, d, p, c, s[h + 10], 23, -1094730640),
                        c = v(c, u, d, p, s[h + 13], 4, 681279174),
                        p = v(p, c, u, d, s[h + 0], 11, -358537222),
                        d = v(d, p, c, u, s[h + 3], 16, -722521979),
                        u = v(u, d, p, c, s[h + 6], 23, 76029189),
                        c = v(c, u, d, p, s[h + 9], 4, -640364487),
                        p = v(p, c, u, d, s[h + 12], 11, -421815835),
                        d = v(d, p, c, u, s[h + 15], 16, 530742520),
                        c = g(c, u = v(u, d, p, c, s[h + 2], 23, -995338651), d, p, s[h + 0], 6, -198630844),
                        p = g(p, c, u, d, s[h + 7], 10, 1126891415),
                        d = g(d, p, c, u, s[h + 14], 15, -1416354905),
                        u = g(u, d, p, c, s[h + 5], 21, -57434055),
                        c = g(c, u, d, p, s[h + 12], 6, 1700485571),
                        p = g(p, c, u, d, s[h + 3], 10, -1894986606),
                        d = g(d, p, c, u, s[h + 10], 15, -1051523),
                        u = g(u, d, p, c, s[h + 1], 21, -2054922799),
                        c = g(c, u, d, p, s[h + 8], 6, 1873313359),
                        p = g(p, c, u, d, s[h + 15], 10, -30611744),
                        d = g(d, p, c, u, s[h + 6], 15, -1560198380),
                        u = g(u, d, p, c, s[h + 13], 21, 1309151649),
                        c = g(c, u, d, p, s[h + 4], 6, -145523070),
                        p = g(p, c, u, d, s[h + 11], 10, -1120210379),
                        d = g(d, p, c, u, s[h + 2], 15, 718787259),
                        u = g(u, d, p, c, s[h + 9], 21, -343485551),
                        c = c + y >>> 0,
                        u = u + _ >>> 0,
                        d = d + b >>> 0,
                        p = p + $ >>> 0
                }
                return endian([c, u, d, p])
            }
        )._ff = function(e, t, r, n, i, o, a) {
            var s = e + (t & r | ~t & n) + (i >>> 0) + a;
            return (s << o | s >>> 32 - o) + t
        }
        ,
        o._gg = function(e, t, r, n, i, o, a) {
            var s = e + (t & n | r & ~n) + (i >>> 0) + a;
            return (s << o | s >>> 32 - o) + t
        }
        ,
        o._hh = function(e, t, r, n, i, o, a) {
            var s = e + (t ^ r ^ n) + (i >>> 0) + a;
            return (s << o | s >>> 32 - o) + t
        }
        ,
        o._ii = function(e, t, r, n, i, o, a) {
            var s = e + (r ^ (t | ~n)) + (i >>> 0) + a;
            return (s << o | s >>> 32 - o) + t
        }
        ,
        o._blocksize = 16,
        o._digestsize = 16,
        e.exports = function(e, r) {
            var n = wordsToBytes(o(e, r));
            return r && r.asBytes ? n : r && r.asString ? bytesToString(n) : bytesToHex(n)
        }
}
));



function bytesToHex(e) {
    for (var t = [], r = 0; r < e.length; r++)
        t.push((e[r] >>> 4).toString(16)),
            t.push((15 & e[r]).toString(16));
    return t.join("")
}

function swapString(e, t) {
    if (e.length % 2)
        return e;
    if (0 === t)
        return e;
    if (e.length === Math.pow(2, t))
        return e.split("").reverse().join();
    var r = e.slice(0, e.length / 2)
        , n = e.slice(e.length / 2);
    return "".concat(swapString(n, t - 1)).concat(swapString(r, t - 1))
}

function getMixinKey(e) {
    var t = [];
    return [46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19, 29, 28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25, 54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52].forEach((function(r) {
            e.charAt(r) && t.push(e.charAt(r))
        }
    )),
        t.join("").slice(0, 32)
}
const imgkey = process.argv[2];
const subkey = process.argv[3];

y = {
    imgKey:imgkey,
    subKey: subkey
}
let result = encWbi({},{},y);

console.log(`{"w_rid": "${result.w_rid}","wts": ${result.wts} }`);