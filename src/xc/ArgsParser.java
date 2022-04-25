package xc;

import java.util.ArrayList;
import java.util.Arrays;


class Args {
	private ArrayList<String> flags;
	private ArrayList<String> args;
	
	Args(String[] flags, String[] args){
		this.flags = new ArrayList<>(Arrays.asList(flags));
		this.args = new ArrayList<>(Arrays.asList(args));;
	}
	
	public ArrayList<String> args() {
		return this.args;
	}
	
	public ArrayList<String> flags() {
		return this.flags;
	}
}

public class ArgsParser {
	private String[] arguments;
	private Args _args;
	ArrayList<String> flags;
	ArrayList<String> args;

	/**
	 * Takes cli arguments and parses them into 'Args' which can be accessed by referencing:
	 * ArgsParser.args() or ArgsParser.flags()
	 * @param args String[]
	 * @throws IllegalArgumentException if not enough Arguments are provided
	 */
	public ArgsParser(String args[]){
		if(args.length < 1) {
			throw new IllegalArgumentException("Not enough Arguments");
		}
		this.arguments = args.clone();
		
		this.parse();
		
		this.flags = this._args.flags();
		this.args = this._args.args();
	}
	
	/**
	 * inserts all args begining with '-' or '--' into a flag arraylist 
	 * and inserts the rest into a args arraylist.
	 * '-' gets replaced and only args which contain a '.' are moved into the args list, everything else is ignored
	 */
	private void parse() {
		ArrayList<String> parsedArgs = new ArrayList<String>();
		ArrayList<String> parsedFlags = new ArrayList<String>();
		
		for(String i : this.arguments) {
			if(i.startsWith("--") || i.startsWith("-")) {
				String temp = i.replace("-", "");
				parsedFlags.add(temp);
			} else if(i.contains(".")) {
				parsedArgs.add(i);
			}
		}
		
		
		String[] finalArgs = parsedArgs.toArray(new String[0]);
		String[] finalFlags = parsedFlags.toArray(new String[0]);
 		Args parsedArguments = new Args(finalArgs, finalFlags);
 		
		this._args =  parsedArguments;
	}
}
