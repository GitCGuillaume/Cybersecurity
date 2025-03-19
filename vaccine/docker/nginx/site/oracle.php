<?php
	$dtb = oci_pconnect("system", "toor", "oracle:1521/FREE");
	if ($dtb) {
		echo "Ok";
	} else {
		die oci_error();
	}
?>

<!DOCTYPE html>
<html>
<head>
	<title>oracle infection</title>
</head
<body>
	<form action="post">
		<input />
		<input type="hidden" />
		<input type="submit" />
	</form>
	<a href="./mysql.php">mysql.php</a>
	<a href="./oracle.php">oracle.php</a>
</body>
</html>
