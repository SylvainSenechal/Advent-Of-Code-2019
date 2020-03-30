const start = 246515
const end = 739105

const solvePart1 = () => {
  let count = 0
  for (let i = start; i < end; i++) {
    if (foundAdjacentDigits(i) && neverDecrease(i)) count++
  }
  return count
}

const solvePart2 = () => {
  let count = 0
  for (let i = start; i < end; i++) {
    if (foundAdjacentDigitsV2(i) && neverDecrease(i)) count++
  }
  return count
}

const foundAdjacentDigits = number => {
  number = number.toString()
  for (let i = 0; i < number.length - 1; i++) {
    if (number[i] === number[i + 1]) return true
  }
  return false
}

const foundAdjacentDigitsV2 = number => {
  number = number.toString()
  lastAdjacent = -1
  for (let i = 0; i < number.length - 1; i++) {
    if (number[i] !== lastAdjacent) {
      if (number[i] === number[i + 1] && number[i] !== number[i + 2]) {
        return true
      } else {
        lastAdjacent = number[i]
      }
    }
  }
  return false
}

const neverDecrease = number => {
  number = number.toString()
  for (let i = 0; i < number.length - 1; i++) {
    if (number[i + 1] < number[i]) return false
  }
  return true
}

console.log(solvePart1())
console.log(solvePart2())
