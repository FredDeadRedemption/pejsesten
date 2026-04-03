export {};

declare global {
  interface Array<T> {
      getRandomValue(): T;
      shuffle(): Array<T>;
      draw(n: number): Array<T>;
      drawFromBottom(n: number): Array<T>;
      removeRandom(): T;
  }
}

// Returns a random value from the array
Array.prototype.getRandomValue = function<T>(): T {
  return this[Math.floor(Math.random() * this.length)];
}

// Returns a shuffled array
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
Array.prototype.drawFromBottom = function<T>(n: number): Array<T> {
  return this.splice(-n, n);
};

// removes a random element from the array and returns it
Array.prototype.removeRandom = function<T>(): T {
  const index = Math.floor(Math.random() * this.length);
  const [removed] = this.splice(index, 1);
  return removed;
};