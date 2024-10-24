#include <fiatluz.h>
#include <stdio.h>

int main(void) {
	Point p1 = point(1, 2);
	printf("C: Point p1: (%d, %d)\n", p1.x, p1.y);
	Point p2 = point(3, 4);
	Point p3;
	p3.x = 5;
	p3.y = 6;

	Path *h = newPath();
	void *h2 = newPath();

	addPointToPath(p1, h);

	for (int i=0; i<10; i++) {
		addPointToPath(p1, h);
		addPointToPath(p2, h);
		addPointToPath(p3, h);
	}

	//Path* h3 = newPathFromZero();
	//addPointToPath(h3, p2);
	//deletePath(h3);

	deletePath(h);
	deletePath(h2);

	return 0;
}
