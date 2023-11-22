(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
  typeof define === 'function' && define.amd ? define(['exports'], factory) :
  (global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global.backend = {}));
})(this, (function (exports) { 'use strict';

  // Unique ID creation requires a high quality random # generator. In the browser we therefore
  // require the crypto API and do not support built-in fallback to lower quality random number
  // generators (like Math.random()).
  let getRandomValues;
  const rnds8 = new Uint8Array(16);
  function rng() {
    // lazy load so that environments that need to polyfill have a chance to do so
    if (!getRandomValues) {
      // getRandomValues needs to be invoked in a context where "this" is a Crypto implementation.
      getRandomValues = typeof crypto !== 'undefined' && crypto.getRandomValues && crypto.getRandomValues.bind(crypto);

      if (!getRandomValues) {
        throw new Error('crypto.getRandomValues() not supported. See https://github.com/uuidjs/uuid#getrandomvalues-not-supported');
      }
    }

    return getRandomValues(rnds8);
  }

  const REGEX = /^(?:[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}|00000000-0000-0000-0000-000000000000)$/i;

  function validate(uuid) {
    return typeof uuid === 'string' && REGEX.test(uuid);
  }

  /**
   * Convert array of 16 byte values to UUID string format of the form:
   * XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
   */

  const byteToHex = [];

  for (let i = 0; i < 256; ++i) {
    byteToHex.push((i + 0x100).toString(16).slice(1));
  }

  function unsafeStringify(arr, offset = 0) {
    // Note: Be careful editing this code!  It's been tuned for performance
    // and works in ways you may not expect. See https://github.com/uuidjs/uuid/pull/434
    return byteToHex[arr[offset + 0]] + byteToHex[arr[offset + 1]] + byteToHex[arr[offset + 2]] + byteToHex[arr[offset + 3]] + '-' + byteToHex[arr[offset + 4]] + byteToHex[arr[offset + 5]] + '-' + byteToHex[arr[offset + 6]] + byteToHex[arr[offset + 7]] + '-' + byteToHex[arr[offset + 8]] + byteToHex[arr[offset + 9]] + '-' + byteToHex[arr[offset + 10]] + byteToHex[arr[offset + 11]] + byteToHex[arr[offset + 12]] + byteToHex[arr[offset + 13]] + byteToHex[arr[offset + 14]] + byteToHex[arr[offset + 15]];
  }

  function parse(uuid) {
    if (!validate(uuid)) {
      throw TypeError('Invalid UUID');
    }

    let v;
    const arr = new Uint8Array(16); // Parse ########-....-....-....-............

    arr[0] = (v = parseInt(uuid.slice(0, 8), 16)) >>> 24;
    arr[1] = v >>> 16 & 0xff;
    arr[2] = v >>> 8 & 0xff;
    arr[3] = v & 0xff; // Parse ........-####-....-....-............

    arr[4] = (v = parseInt(uuid.slice(9, 13), 16)) >>> 8;
    arr[5] = v & 0xff; // Parse ........-....-####-....-............

    arr[6] = (v = parseInt(uuid.slice(14, 18), 16)) >>> 8;
    arr[7] = v & 0xff; // Parse ........-....-....-####-............

    arr[8] = (v = parseInt(uuid.slice(19, 23), 16)) >>> 8;
    arr[9] = v & 0xff; // Parse ........-....-....-....-############
    // (Use "/" to avoid 32-bit truncation when bit-shifting high-order bytes)

    arr[10] = (v = parseInt(uuid.slice(24, 36), 16)) / 0x10000000000 & 0xff;
    arr[11] = v / 0x100000000 & 0xff;
    arr[12] = v >>> 24 & 0xff;
    arr[13] = v >>> 16 & 0xff;
    arr[14] = v >>> 8 & 0xff;
    arr[15] = v & 0xff;
    return arr;
  }

  const randomUUID = typeof crypto !== 'undefined' && crypto.randomUUID && crypto.randomUUID.bind(crypto);
  const native = {
    randomUUID
  };

  function v4(options, buf, offset) {
    if (native.randomUUID && !buf && !options) {
      return native.randomUUID();
    }

    options = options || {};
    const rnds = options.random || (options.rng || rng)(); // Per 4.4, set bits for version and `clock_seq_hi_and_reserved`

    rnds[6] = rnds[6] & 0x0f | 0x40;
    rnds[8] = rnds[8] & 0x3f | 0x80; // Copy bytes to buffer, if provided

    if (buf) {
      offset = offset || 0;

      for (let i = 0; i < 16; ++i) {
        buf[offset + i] = rnds[i];
      }

      return buf;
    }

    return unsafeStringify(rnds);
  }

  function getDefaultExportFromCjs (x) {
  	return x && x.__esModule && Object.prototype.hasOwnProperty.call(x, 'default') ? x['default'] : x;
  }

  // base-x encoding / decoding
  // Copyright (c) 2018 base-x contributors
  // Copyright (c) 2014-2018 The Bitcoin Core developers (base58.cpp)
  // Distributed under the MIT software license, see the accompanying
  // file LICENSE or http://www.opensource.org/licenses/mit-license.php.
  function base (ALPHABET) {
    if (ALPHABET.length >= 255) { throw new TypeError('Alphabet too long') }
    var BASE_MAP = new Uint8Array(256);
    for (var j = 0; j < BASE_MAP.length; j++) {
      BASE_MAP[j] = 255;
    }
    for (var i = 0; i < ALPHABET.length; i++) {
      var x = ALPHABET.charAt(i);
      var xc = x.charCodeAt(0);
      if (BASE_MAP[xc] !== 255) { throw new TypeError(x + ' is ambiguous') }
      BASE_MAP[xc] = i;
    }
    var BASE = ALPHABET.length;
    var LEADER = ALPHABET.charAt(0);
    var FACTOR = Math.log(BASE) / Math.log(256); // log(BASE) / log(256), rounded up
    var iFACTOR = Math.log(256) / Math.log(BASE); // log(256) / log(BASE), rounded up
    function encode (source) {
      if (source instanceof Uint8Array) ; else if (ArrayBuffer.isView(source)) {
        source = new Uint8Array(source.buffer, source.byteOffset, source.byteLength);
      } else if (Array.isArray(source)) {
        source = Uint8Array.from(source);
      }
      if (!(source instanceof Uint8Array)) { throw new TypeError('Expected Uint8Array') }
      if (source.length === 0) { return '' }
          // Skip & count leading zeroes.
      var zeroes = 0;
      var length = 0;
      var pbegin = 0;
      var pend = source.length;
      while (pbegin !== pend && source[pbegin] === 0) {
        pbegin++;
        zeroes++;
      }
          // Allocate enough space in big-endian base58 representation.
      var size = ((pend - pbegin) * iFACTOR + 1) >>> 0;
      var b58 = new Uint8Array(size);
          // Process the bytes.
      while (pbegin !== pend) {
        var carry = source[pbegin];
              // Apply "b58 = b58 * 256 + ch".
        var i = 0;
        for (var it1 = size - 1; (carry !== 0 || i < length) && (it1 !== -1); it1--, i++) {
          carry += (256 * b58[it1]) >>> 0;
          b58[it1] = (carry % BASE) >>> 0;
          carry = (carry / BASE) >>> 0;
        }
        if (carry !== 0) { throw new Error('Non-zero carry') }
        length = i;
        pbegin++;
      }
          // Skip leading zeroes in base58 result.
      var it2 = size - length;
      while (it2 !== size && b58[it2] === 0) {
        it2++;
      }
          // Translate the result into a string.
      var str = LEADER.repeat(zeroes);
      for (; it2 < size; ++it2) { str += ALPHABET.charAt(b58[it2]); }
      return str
    }
    function decodeUnsafe (source) {
      if (typeof source !== 'string') { throw new TypeError('Expected String') }
      if (source.length === 0) { return new Uint8Array() }
      var psz = 0;
          // Skip and count leading '1's.
      var zeroes = 0;
      var length = 0;
      while (source[psz] === LEADER) {
        zeroes++;
        psz++;
      }
          // Allocate enough space in big-endian base256 representation.
      var size = (((source.length - psz) * FACTOR) + 1) >>> 0; // log(58) / log(256), rounded up.
      var b256 = new Uint8Array(size);
          // Process the characters.
      while (source[psz]) {
              // Decode character
        var carry = BASE_MAP[source.charCodeAt(psz)];
              // Invalid character
        if (carry === 255) { return }
        var i = 0;
        for (var it3 = size - 1; (carry !== 0 || i < length) && (it3 !== -1); it3--, i++) {
          carry += (BASE * b256[it3]) >>> 0;
          b256[it3] = (carry % 256) >>> 0;
          carry = (carry / 256) >>> 0;
        }
        if (carry !== 0) { throw new Error('Non-zero carry') }
        length = i;
        psz++;
      }
          // Skip leading zeroes in b256.
      var it4 = size - length;
      while (it4 !== size && b256[it4] === 0) {
        it4++;
      }
      var vch = new Uint8Array(zeroes + (size - it4));
      var j = zeroes;
      while (it4 !== size) {
        vch[j++] = b256[it4++];
      }
      return vch
    }
    function decode (string) {
      var buffer = decodeUnsafe(string);
      if (buffer) { return buffer }
      throw new Error('Non-base' + BASE + ' character')
    }
    return {
      encode: encode,
      decodeUnsafe: decodeUnsafe,
      decode: decode
    }
  }
  var src = base;

  const basex = src;
  const ALPHABET = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';

  var bs58 = basex(ALPHABET);

  const bs58$1 = /*@__PURE__*/getDefaultExportFromCjs(bs58);

  function generateId() {
    const uuid = parse(v4());
    return bs58$1.encode(uuid);
  }
  class Api {
    constructor(fetch) {
      this.fetch = fetch;
    }
    async #sendCreateCommand(url, command) {
      const res = await this.fetch(url, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(command)
      });
      return await res.json();
    }
    async #sendCommand(url, command) {
      const res = await this.fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(command)
      });
      return await res.json();
    }
    async #get(url) {
      const res = await this.fetch(url);
      return await res.json();
    }
    async internalCreateUser(id, profile) {
      let command = { profile };
      return await this.#sendCreateCommand(`/api/users/${id}`, command);
    }
    async internalFakeLogin(userId) {
      const res = await this.fetch(`/api/fake-login/${userId}`, {
        method: "POST"
      });
      return await res.json();
    }
    async telegramLogin(data) {
      return await this.#sendCommand(`/api/login/telegram`, data);
    }
    async getMe() {
      return await this.#get("/api/users/me");
    }
    async getUserProfile(id) {
      return await this.#get(`/api/users/${id}/profile`);
    }
    async getUserGroups(id) {
      return await this.#get(`/api/users/${id}/groups`);
    }
    async createGroup(id, creation) {
      return await this.#sendCreateCommand(`/api/groups/${id}`, creation);
    }
    async getGroupTickets(id) {
      return await this.#get(`/api/groups/${id}/tickets`);
    }
    async getGroup(id) {
      return await this.#get(`/api/groups/${id}`);
    }
    async addGroupMember(id, new_member) {
      let command = { type: "AddMember", new_member };
      return await this.#sendCommand(`/api/groups/${id}`, command);
    }
    async removeGroupMember(id, removed_member) {
      let command = { type: "RemoveMember", removed_member };
      return await this.#sendCommand(`/api/groups/${id}`, command);
    }
    async changeGroupTitle(id, new_title) {
      let command = { type: "ChangeTitle", new_title };
      return await this.#sendCommand(`/api/groups/${id}`, command);
    }
    async createTicket(id, creation) {
      return await this.#sendCreateCommand(`/api/tickets/${id}`, creation);
    }
    async getTicket(id) {
      return await this.#get(`/api/tickets/${id}`);
    }
    async getOwnedTickets() {
      return await this.#get(`/api/tickets/owned`);
    }
    async getAssignedTickets() {
      return await this.#get(`/api/tickets/assigned`);
    }
    async sendTicketMessage(id, message) {
      let command = { type: "SendTicketMessage", ...message };
      return await this.#sendCommand(`/api/tickets/${id}`, command);
    }
    async changeTicketStatus(id, new_status) {
      let command = { type: "ChangeStatus", new_status };
      return await this.#sendCommand(`/api/tickets/${id}`, command);
    }
    async changeTicketAssignee(id, new_assignee) {
      let command = { type: "ChangeAssignee", new_assignee };
      return await this.#sendCommand(`/api/tickets/${id}`, command);
    }
    async searchTickets(q) {
      return await this.#get(`/api/search/tickets?q=${q}`);
    }
    async searchUsers(q) {
      return await this.#get(`/api/search/users?q=${q}`);
    }
    async searchGroups(q) {
      return await this.#get(`/api/search/groups?q=${q}`);
    }
    async initiateUpload(metadata) {
      const res = await this.fetch(`/api/upload/initiate`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(metadata)
      });
      return await res.json();
    }
    async finalizeUpload(id) {
      const res = await this.fetch(`/api/upload/${id}/finalize`, {
        method: "POST"
      });
      return await res.json();
    }
    getUploadFileUrl(id) {
      return `/api/upload/${id}/file`;
    }
  }

  exports.Api = Api;
  exports.generateId = generateId;

  Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

}));
