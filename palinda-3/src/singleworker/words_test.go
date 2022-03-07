package main

import (
	"io/ioutil"
	"reflect"
	"testing"
)

// Test that the word count on loremipsum.txt is correct
func TestWordCountOnLoremIpsum(t *testing.T) {
	data, _ := ioutil.ReadFile(DataFile)

	expected := map[string]int{"a": 932, "ac": 1224, "accumsan": 402,
		"adipiscing": 958, "aenean": 474, "aliqua": 2, "aliquam": 1148, "aliquet": 924,
		"amet": 2222, "ante": 152, "arcu": 1168, "at": 1572, "auctor": 428, "augue": 528,
		"bibendum": 610, "blandit": 572, "commodo": 672, "condimentum": 398,
		"congue": 366, "consectetur": 648, "consequat": 556, "convallis": 450,
		"cras": 716, "cum": 104, "curabitur": 170, "cursus": 878, "dapibus": 82,
		"diam": 1324, "dictum": 456, "dictumst": 190, "dignissim": 586, "dis": 128,
		"do": 2, "dolor": 664, "dolore": 2, "donec": 658, "dui": 642, "duis": 564,
		"egestas": 1258, "eget": 1786, "eiusmod": 2, "eleifend": 296, "elementum": 854,
		"elit": 796, "enim": 1522, "erat": 374, "eros": 216, "est": 802, "et": 1608,
		"etiam": 442, "eu": 1248, "euismod": 456, "facilisi": 350, "facilisis": 566,
		"fames": 274, "faucibus": 1082, "felis": 532, "fermentum": 592, "feugiat": 686,
		"fringilla": 394, "fusce": 172, "gravida": 724, "habitant": 244,
		"habitasse": 206, "hac": 200, "hendrerit": 248, "iaculis": 348, "id": 1900,
		"imperdiet": 452, "in": 2290, "incididunt": 2, "integer": 504, "interdum": 484,
		"ipsum": 728, "justo": 376, "labore": 2, "lacinia": 182, "lacus": 784,
		"laoreet": 354, "lectus": 884, "leo": 702, "libero": 458, "ligula": 72,
		"lobortis": 388, "lorem": 584, "luctus": 234, "maecenas": 514, "magna": 566,
		"magnis": 122, "malesuada": 604, "massa": 1144, "mattis": 796, "mauris": 1194,
		"metus": 270, "mi": 772, "molestie": 374, "mollis": 208, "montes": 108,
		"morbi": 984, "mus": 100, "nam": 250, "nascetur": 100, "natoque": 102, "nec": 728,
		"neque": 898, "netus": 260, "nibh": 949, "nisi": 576, "nisl": 912, "non": 1170,
		"nulla": 1116, "nullam": 410, "nunc": 1636, "odio": 842, "orci": 742,
		"ornare": 676, "parturient": 120, "pellentesque": 1408, "penatibus": 112,
		"pharetra": 798, "phasellus": 294, "placerat": 386, "platea": 210, "porta": 307,
		"porttitor": 464, "posuere": 484, "potenti": 90, "praesent": 282, "pretium": 620,
		"proin": 540, "pulvinar": 624, "purus": 930, "quam": 892, "quis": 1292,
		"quisque": 390, "rhoncus": 516, "ridiculus": 100, "risus": 1132, "rutrum": 228,
		"sagittis": 656, "sapien": 422, "scelerisque": 880, "sed": 2728, "sem": 514,
		"semper": 562, "senectus": 266, "sit": 2212, "sociis": 100, "sodales": 246,
		"sollicitudin": 364, "suscipit": 194, "suspendisse": 554, "tellus": 1228,
		"tempor": 444, "tempus": 422, "tincidunt": 1080, "tortor": 1020,
		"tristique": 676, "turpis": 1010, "ullamcorper": 670, "ultrices": 838,
		"ultricies": 522, "urna": 944, "ut": 1846, "varius": 482, "vehicula": 86,
		"vel": 996, "velit": 756, "venenatis": 501, "vestibulum": 588, "vitae": 1634,
		"vivamus": 184, "viverra": 1292, "volutpat": 948, "vulputate": 686}

	if actual := WordCount(string(data)); !reflect.DeepEqual(actual, expected) {
		t.Errorf("expected: %v\nactual: %v", expected, actual)
	}

}
