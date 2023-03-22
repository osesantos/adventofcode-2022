package day1

object day2 {
  def part1(fileName: String): String = {
    getTotalScore(utils.ioutils.readFileLines(fileName)).toString
  }

  /*
  Opponent
  A - 1
  B - 2
  c - 3

  Me
  X - 1
  Y - 2
  z - 3

  Lost - 0
  Draw - 3
  Won  - 6
  */
  private def getTotalScore(lines: List[String]): Int = lines.parseToScores.parseToFinalScore.sum

  private def getMatchScore(a: Int, b: Int): Int = {
    if a > b then return b + 0
    if a == b then return b + 3
    else return b + 6
  }

  extension (lines: List[String])
    private def parseToScores: List[(Int, Int)] = lines.map(l => (l.substring(0, 1).parseToScore, l.substring(2, 3).parseToScore))

  extension (input: String)
    private def parseToScore: Int = input match
      case "A" => 1
      case "B" => 2
      case "C" => 3
      case "X" => 1
      case "Y" => 2
      case "Z" => 3

  extension (scores: List[(Int, Int)])
    private def parseToFinalScore: List[Int] = scores.map((a, b) => getMatchScore(a, b))



}
