# srm (safe remove) - garbage collector
# Put file into garbage collector (a designated path).
# Garbage Collector (/collector) must be in a temp location allowed by the
# system (Linux, MacOS, Sun, Windows, etc.)
# If user wants to add a new path srm will error: already created collector
# So srm must first ask if in default path the collector is there. If not there
# then create it and store file.
# Must be able to see collector information ( current size, files stored, file
# sizes in bytes (kb, Mb, Gb, Tb), time of deletion, original location)
# They should also be able to request collector information such as location,
# files contained, etc.
# User can delete stuff contained in collector without needing to
# go into the exact file path of collector.
# User can delete stuff contained in collector with remote
# (i.e srm --unlink file)
# Can restore files to their original location
# They can also unlink (delete) files in batch, srm will prompt for
# confirmatiom when this happens.
# Does everything that rm does but safe (implement in Rust)
#
# Example of usage:
#
# srm myfile.txt
# srm --info (equivalent to ls -la /path/to/collector but with information about
#   the parent directory (/collector) size, files contained, + files contained)
#   doesn't show ./ and ../ in output.
# srm --unlink myfile.txt (removes file)
# srm --unlink * (removes all files contained in collector)
# Are you sure you want to delete [file]? y/n
# srm --unlink collector (removes the collector from the system)
