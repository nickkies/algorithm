# "./"과 "../" 이 포함된 파일 경로를 "./"과 "../"이 없는 유닉스 파일 경로로 바꾸시오.
# "./"는 현재의 위치를 뜻하고, "../"는 상위 디렉토리를 뜻합니다.
# Given a file path containing "./" and "../", convert the path to a unix standard file path that does not contain "./" and "../".
#
# input: "/usr/bin/../"
# output: "/usr/"
#
# input: "/usr/./bin/./test/../"
# output: "/usr/bin/"


def standardize(path):
    stack = []
    path = path.split("/")

    print(path)

    for segment in path:
        if segment == ".":
            continue

        elif segment == "..":
            if len(stack) > 1:
                stack.pop()

        else:
            stack.append(segment)

    return "/".join(stack)


input1 = "/usr/bin/../"
input2 = "/usr/./bin/./test/../"

output1 = standardize(input1)
output2 = standardize(input2)

print(output1)
print(output2)
