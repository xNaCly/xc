enum MODES {
	CHARS, WORDS, LINES, ALL,
}

class File {
	String name;
	String path;
	int chars;
	int words;
	int lines;

	public File(String path) {
		this.path = path;
		String[] temp = path.split("/");
		this.name = temp[temp.length - 1]; // get last item of path split in l.14
		this.workFile();
	}

	private void workFile() {
		// read file and assigning read values to variables
		this.chars = 25;
		this.words = 25;
		this.lines = 25;
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
		String[] args_ = { "../test.h" };
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
			File f = new File(a);
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
				System.out.printf("%d %d %d %s %n", f.chars, f.words, f.lines, f.name);
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
				System.out.printf("%d %d %d total %n", totalChars, totalWords, totalLines);
			}
		}
	}

}
