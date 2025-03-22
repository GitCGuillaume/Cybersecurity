<?php
try {
	#http://127.0.0.1/mysql.php?a=)'; drop table list;'
	$dtb = new PDO('sqlite:/var/www/site/test');
	echo "Connected to database Sqlite";
	var_dump($dtb);
	$type = 'abc';
	if ($_GET['a']) {
		$type = $_GET['a'];
	}
	$query_str = "SELECT * from list where vals = '$type'";
	$res = $dtb->query($query_str);
	echo '<pre> ----- ', var_dump($res->fetchAll()) , ' ----- </pre>';
	$res->closeCursor();
	if ($_POST['txt']) {
		echo 'post_txt: ', $_POST['txt'];
		$post = $_POST['txt'];
		$insert = "INSERT INTO list(vals) VALUES ('$post')";
		$res = $dtb->prepare($insert);
		$res2 = $res->execute();
		echo 'res: <pre>', var_dump($res) , '</pre>';
		$res->closeCursor();
	}
	if ($_POST['hid']) {
		echo 'post_hid: ', $_POST['hid'];
		$post = $_POST['hid'];
		$insert = "INSERT INTO list(vals) VALUES ('$post')";
		$res = $dtb->prepare($insert);
		$res2 = $res->execute();
		echo 'res: <pre>', var_dump($res) , '</pre>';
		$res->closeCursor();
	}
	$query_str = "SELECT * from list";
	$res = $dtb->query($query_str);
	echo '<pre>', var_dump($res->fetchAll()) , '</pre>';
	$res->closeCursor();
	
}
catch (PDOException $err) {
	die($err);
}
?>

<!DOCTYPE html>
<html>
<head>
	<title>Sqlite infection</title>
</head
<body>
	<form action="sqlite.php" method="post">
		<input name="txt" type="text" />
		<input name="hid"  type="hidden" />
		<input type="submit" />
	</form>
	<a href="./mysql.php">mysql.php</a>
	<a href="./mysql.php?a=abc">mysql.php?a=abc</a>
	<a href="./sqlite.php">sqlite.php</a>
	<a href="./sqlite.php?a=abc">sqlite.php?a=abc</a>
</body>
</html>
