import scala.collection.mutable.Stack
import java.{util => ju}
import scala.collection.mutable

enum Instruction:
  case Add(x: Byte)
  case Move(x: Int)
  case Loop(end: Int)
  case End(start: Int)
  case Print
  case Read

object Instruction:
  def parse(inst: Char) = inst match
    case '+' => Some(Instruction.Add(1))
    case '-' => Some(Instruction.Add(-1))
    case '>' => Some(Instruction.Move(1))
    case '<' => Some(Instruction.Move(-1))
    case '.' => Some(Instruction.Print)
    case ',' => Some(Instruction.Read)
    case '[' => Some(Instruction.Loop(0))
    case ']' => Some(Instruction.End(0))
    case _   => None

  def parseString(prog: String) =
    val parsed = for i <- prog yield parse(i)
    var mapped = parsed.filter(_.isDefined).map(_.get).toArray
    applyLoopIndexes(mapped)
    mapped.toVector

  private def findLoopIndexes(insts: IndexedSeq[Instruction]) =
    var stack = Stack[Int]()
    var out = Stack[(Int, Int)]()
    for (inst, i) <- insts.zipWithIndex do
      inst match
        case Loop(_) => stack.push(i)
        case End(_) => out.push((stack.pop(), i))
        case _       => ()
    out.toVector

  def applyLoopIndexes(insts: mutable.IndexedSeq[Instruction]) =
    for (start, end) <- findLoopIndexes(insts.toIndexedSeq) do
      insts(start) = Loop(end)
      insts(end) = End(start)


trait Optimizer:
  def optimize(input: IndexedSeq[Instruction]): IndexedSeq[Instruction]

object Grouper extends Optimizer:
  import Instruction.*

  def optimize(input: IndexedSeq[Instruction]): IndexedSeq[Instruction] =
    var prev: Option[Instruction] = None
    val nullable = for i <- input yield (i, prev) match
      case (Add(n), Some(Add(o))) =>
        prev = Some(Add((n + o).toByte))
        None
      case (Move(n), Some(Move(o))) =>
        prev = Some(Move(n + o))
        None
      case (n, None) =>
        prev = Some(n)
        None
      case (n, o) =>
        prev = Some(n)
        o
    var mapped = (nullable.filter(_.isDefined).map(_.get) :+ prev.get).toArray
    applyLoopIndexes(mapped)
    mapped
