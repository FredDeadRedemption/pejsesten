export {};

declare global {
  interface Array<T> {
      shuffle(): Array<T>;
      draw(n: number): Array<T>;
      dredge(n: number): Array<T>;
  }
}

// returns a shuffled array ( fisher-yates algorithm )
Array.prototype.shuffle = function() {
  for (let i = this.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [this[i], this[j]] = [this[j], this[i]];
  }
  return this;
};

// removes n elements from the start of the array and returns them
Array.prototype.draw = function<T>(n: number): Array<T> {
  return this.splice(0, n);
};

// removes n elements from the end of the array and returns them
Array.prototype.dredge = function<T>(n: number): Array<T> {
  return this.splice(-n, n);
};