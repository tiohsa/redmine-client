export default class Result {
    constructor(left = undefined, right = undefined) {
        this.left = left;
        this.right = right;
        this.isRight = this.right ? true : false;
        this.isLeft = this.left ? true : false;
    }
}
