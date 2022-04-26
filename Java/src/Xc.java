import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

enum MODES {
	CHARS, WORDS, LINES, ALL,
}

class Xfile {
	String name;
	String path;
	int chars = 0;
	int words= 0;
	int lines = 0;

	public Xfile(String path) {
		this.path = path;
		String[] temp = path.split("/");
		this.name = temp[temp.length - 1]; // get last item of path split in l.14
		this.readFile();
	}
	

	private void readFile() {
		try {
			File f = new File(this.path);
			Scanner sc = new Scanner(f);
			chars = (int)f.length();
			while(sc.hasNext()) {
				String data = sc.nextLine();
				words += data.split(" ").length;
				lines++;
			}
			sc.close();
		} catch (FileNotFoundException e) {
			System.out.println(this.name + " couldnt be read.");
			e.printStackTrace();
		}
		
	}
}

public class Xc {
	static MODES mode = MODES.ALL;
	static int totalChars = 0;
	static int totalWords = 0;
	static int totalLines = 0;

	public static void main(String[] args) {
		start(args);

	}

	private static void start(String[] args) {
		String[] args_ = { "/home/teo/test.test" };
		ArgsParser ap = new ArgsParser(args_);

		for (String a : ap.flags) {
			switch (a) {
			case "version":
			case "v":
				System.out.println("xc - version: 1");
				return;
			case "help":
			case "h":
				System.out.println(
						"Usage: \n\txc [FILES] [OPTIONS]\n \n-m  --chars \n\t Print characters in file\n \n-l  --lines \n\t Print lines in file\n \n-w  --words \n\t Print words in file\n");
				return;
			case "lines":
			case "l":
				mode = MODES.LINES;
				break;
			case "chars":
			case "m":
				mode = MODES.CHARS;
				break;
			case "words":
			case "w":
				mode = MODES.WORDS;
				break;
			default:
				throw new IllegalArgumentException("Argument '" + a + "' unknown.");
			}
		}

		for (String a : ap.args) {
			Xfile f = new Xfile(a);
			totalChars += f.chars;
			totalWords += f.words;
			totalLines += f.lines;
			switch (mode) {
			case LINES:
				System.out.printf("%d %s %n", f.lines, f.name);
				break;
			case CHARS:
				System.out.printf("%d %s %n", f.chars, f.name);
				break;
			case WORDS:
				System.out.printf("%d %s %n", f.words, f.name);
				break;
			default:
				System.out.printf("%d %d %d %s %n", f.lines, f.words, f.chars, f.name);
			}
		}
		if (ap.args.size() > 1) {
			switch (mode) {
			case LINES:
				System.out.printf("%d total %n", totalLines);
				break;
			case CHARS:
				System.out.printf("%d total %n", totalChars);
				break;
			case WORDS:
				System.out.printf("%d total %n", totalWords);
				break;
			default:
				System.out.printf("%d %d %d total %n", totalLines, totalWords, totalLines);
			}
		}
	}

}
