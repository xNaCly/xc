package xc;

public class Xc {
	
	public static void main(String[] args) {
		String[] args_ = {"-l", "--help", "file.txt"};
		ArgsParser ap = new ArgsParser(args_);
		
		System.out.println(ap.args.toString() + ap.flags.toString());
	}
}
