.PHONY: npminit forcenpm builddir css scripts images slickgrid package-for-server rebuild-clean clean

BUILDDIR := build
PROJECTDATA := "../../project-data"

all: rebuild-clean css scripts images package-for-server

# Node v8.7.0 fails on redundant `npm install` due to package error.
npminit:
	if [ ! -d "node_modules" ]; then npm install; fi

forcenpm:
	npm install

builddir:
	mkdir -p '${BUILDDIR}'
	mkdir -p '${BUILDDIR}/css'
	mkdir -p '${BUILDDIR}/images'
	mkdir -p '${BUILDDIR}/scripts'

css: builddir npminit
	npm run sass

images: builddir
	cp ${PROJECTDATA}/logo-on-white.svg "${BUILDDIR}/images/logo-compressed.svg"
	cp images/select_dropdown_red.png "${BUILDDIR}/images/"
	cp ${PROJECTDATA}/favicon-32x32.ico "${BUILDDIR}/images/favicon.ico"
	cp ${PROJECTDATA}/apple-touch-icon.png "${BUILDDIR}/images/"

scripts: builddir npminit slickgrid
	npm run webpack

SLICKGRID := "node_modules/slickgrid"
slickgrid: builddir
	cat "${SLICKGRID}/lib/jquery-1.11.2.min.js" \
	    "${SLICKGRID}/lib/jquery.event.drag-2.3.0.js" \
	    "${SLICKGRID}/lib/jquery.event.drop-2.3.0.js" \
	    "${SLICKGRID}/slick.core.js" \
	    "${SLICKGRID}/slick.grid.js" \
	    | ./try-uglify "${BUILDDIR}/scripts/slickgrid.min.js"

package-for-server:
	# Rewrite all the filenames to contain their hash.
	./apply-md5
	# Gzip the contents. The hashed filename is not the hash of the gzip,
	# but the hash of the gzip's contents when decompressed.
	./apply-gzip


# Cleans files that may cause `make` to be non-idempotent.
rebuild-clean:
	rm -rf build

# Fully cleans the working directory.
clean:
	rm -rf build node_modules
