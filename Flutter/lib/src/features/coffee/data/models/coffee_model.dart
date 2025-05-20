
import 'package:coffe_shop/src/features/coffee/domain/entities/coffee_entity.dart';

class CoffeeModel extends CoffeeEntity {


  CoffeeModel();

  factory CoffeeModel.fromJson(Map<String, dynamic> json) => CoffeeModel(

  );

  CoffeeEntity toEntity() {
    return CoffeeEntity();
  }
}