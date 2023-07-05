/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function (s) {
    let result = 0;
    let i = 0;
    while (i < s.length) {
        let current = s[i];
        let next = s[i + 1];
        if (current === "I" && (next === "V" || next === "X")) {
            result -= 1;
        } else if (current === "X" && (next === "L" || next === "C")) {
            result -= 10;
        } else if (current === "C" && (next === "D" || next === "M")) {
            result -= 100;
        } else if (current === "M" && next === "M") {
            result += 1000;
        } else if (current === "M") {
            result += 1000;
        } else if (current === "D") {
            result += 500;
        } else if (current === "C") {
            result += 100;
        } else if (current === "L") {
            result += 50;
        } else if (current === "X") {
            result += 10;
        } else if (current === "V") {
            result += 5;
        } else if (current === "I") {
            result += 1;
        }
        i++;
    }
    return result;
};
