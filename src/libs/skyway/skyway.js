import _Peer from "skyway-js";

export class Peer extends _Peer {
    constructor(key) {
        super({ key: key, debug: 0 });
    }
}