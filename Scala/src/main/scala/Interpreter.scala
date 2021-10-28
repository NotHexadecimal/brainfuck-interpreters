import scala.collection.mutable.ArrayBuffer
import Instruction.*

trait Runner:
  def prepareCode(program: IndexedSeq[Instruction]): Unit
  def runProgram(input: String = ""): String

class Interpreter() extends Runner:
  var mem: ArrayBuffer[Byte] = ArrayBuffer.fill(30000)(0)
  private var prog = Vector[Instruction]()

  def prepareCode(program: IndexedSeq[Instruction]) =
    prog = program.toVector

  def runProgram(input: String): String =
    var ptr = 0
    var inputPtr = 0
    var progPtr = 0
    var output = String()

    while progPtr < prog.length do
      prog(progPtr) match
        case Add(x)     => mem(ptr) = (mem(ptr) + x).toByte
        case Move(n)    => ptr += n
        case Loop(jump) => if mem(ptr) == 0 then progPtr = jump - 1
        case End(jump)  => if mem(ptr) != 0 then progPtr = jump - 1
        case Print =>
          output += mem(ptr).toChar
        case Read =>
          mem(ptr) = input(inputPtr).toByte
          inputPtr = inputPtr + 1
      progPtr += 1

    output
