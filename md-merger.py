import argparse


def read_file(file_path: str) -> str:
    with open(file_path, 'r') as file:
        return file.read()


def write_file(file_path: str, content: str) -> None:
    with open(file_path, 'w') as file:
        file.write(content)


def process_file(file_path: str) -> str:
    content = read_file(file_path).splitlines()
    # find link to .md file
    for i, line in enumerate(content):
        if line.startswith('[') and line.endswith('.md)'):
            link = line.split('(')[1][:-1]
            content[i] = process_file(file_path[:file_path.rindex('/')] + '/' + link.replace('%20', ' '))
    return '\n'.join(content)


def main():
    parser = argparse.ArgumentParser(description='Merge markdown files into one markdown file')
    parser.add_argument('folder', type=str, help='Folder containing markdown files')
    parser.add_argument('start', type=str, help='Starting file name')
    parser.add_argument('output', type=str, help='Output file name')
    args = parser.parse_args()

    content = process_file(f'{args.folder}/{args.start}')
    write_file(f'{args.folder}/{args.output}', content)


if __name__ == '__main__':
    main()
