import java.io.PrintWriter

@main def hello(args: String*) =
  val reader = try
    io.Source.fromFile(args(0))
  catch
    case _ =>
      println("No file provided")
      scala.sys.exit(1)
  val prog = reader.mkString
  reader.close

  val input = try args(2) catch case _ => ""

  var runner = Interpreter()
  val insts = Instruction.parseString(prog)
  val opt = Grouper.optimize(insts)

  runner.prepareCode(opt)
  val output = runner.runProgram(input)
  writeOutput(try Some(args(1)) catch _ => None, output)

def writeOutput(path: Option[String], out: String) = path match
  case None       => print(out)
  case Some("-")  => print(out)
  case Some(path) => PrintWriter(path).write(out)
